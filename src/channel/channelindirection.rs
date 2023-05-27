use crate::common::satoshi::Satoshi as Satoshi;
use crate::common::fee::{Fee as Fee, FeeType as FeeType};
use crate::common::timestamp::{Timestamp as Timestamp, self};
use crate::common::params as params;
use crate::common::errortypes::ErrorType as ErrorType;

use super::htlc::Htlc as Htlc;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::os::linux::raw;

#[derive(Debug)]
pub struct ChannelInDirection {
    pub upfront_fee: Fee,
    pub success_fee: Fee,
    pub deliberately_fail_prob: f64,    // can I define a sub-type of float for probabilities to check 0<=x<=1?
    pub spoofing_error_type: ErrorType,
    // we don't store num_slots separately
    // it's better to obtain this data from the slots queue directly
    slots: PriorityQueue<Htlc, Reverse<Timestamp>>,
}

impl ChannelInDirection {

    pub fn new(
        num_slots: Option<usize>,
        upfront_fee: Option<Fee>,
        success_fee: Option<Fee>,
        deliberately_fail_prob: Option<f64>,
        spoofing_error_type: Option<ErrorType>
    ) -> Self {
        ChannelInDirection {
            upfront_fee: upfront_fee.unwrap_or(Fee::default()),
            success_fee: success_fee.unwrap_or(Fee::default()),
            deliberately_fail_prob: deliberately_fail_prob.unwrap_or(0.0),
            spoofing_error_type: spoofing_error_type.unwrap_or(ErrorType::LowBalance),
            slots: Self::create_slots_queue(num_slots),
            }
    }

    fn new_default() -> Self {
        Self::new(None, None, None, None, None)
    }

    fn create_slots_queue(num_slots: Option<usize>) -> PriorityQueue<Htlc, Reverse<Timestamp>> {
        match num_slots {
            Some(num_slots) => {
                assert!(num_slots <= params::MAX_NUM_SLOTS,
                "Can't have more than {} slots per channel direction, got {}", params::MAX_NUM_SLOTS, num_slots);
                assert!(num_slots > 0,
                "Can't have zero slots in a channel direction! If the channel direction is disabled, set None in Channel.");                PriorityQueue::with_capacity(num_slots)
            },
            None => PriorityQueue::with_capacity(params::MAX_NUM_SLOTS),
        }
    }

    fn reset_slots(&mut self, num_slots: Option<usize>) {
        // create new priority queue for num_slots HTLCs
        self.slots = Self::create_slots_queue(num_slots);
    }

    pub fn set_fee(&mut self, fee_type: FeeType, fee: Fee) {
        match fee_type {
            FeeType::Success => self.success_fee = fee,
            FeeType::Upfront => self.upfront_fee = fee,
        }
    }

    pub fn requires_fee(&self, fee_type: FeeType, amount: &Satoshi) -> Satoshi {
        match fee_type {
            FeeType::Success => self.success_fee.apply(&amount),
            FeeType::Upfront => self.upfront_fee.apply(&amount),
        }
    }

    pub fn requires_total_fee(&self, amount: &Satoshi) -> Satoshi {
        // assume both fees are calculated based on the total amount
        let success_fee = self.requires_fee(FeeType::Success, &amount);
        let upfront_fee = self.requires_fee(FeeType::Upfront, &amount);
        success_fee.add(&upfront_fee)
    }

    pub fn all_slots_free(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn get_num_slots_busy(&self) -> usize {
		// Get the number of HTLCs currently in the queue.
		// Note: some HTLCs may be outdated!
        self.slots.len()
    }

    pub fn get_num_slots_free(&self) -> usize {
        // FIXME: is this correct?
        // priority_queue.capacity():
        // Returns the number of elements the internal map can hold without reallocating.
        // This number is a lower bound; the map might be able to hold more, 
        // but is guaranteed to be able to hold at least this many.
        self.slots.capacity() - self.get_num_slots_busy()
    }

    pub fn all_slots_busy(&self) -> bool {
        self.get_num_slots_free() == 0
    }

    pub fn push_htlc(&mut self, resolution_time: Timestamp, htlc: Htlc) {
        // it's ok to consume HTLC here
        // comment from Python version: the queue must not be full, we must have ensured this earlier
        // TODO: rethink this logic?
        assert!(!self.all_slots_busy());
        self.slots.push(htlc, Reverse(resolution_time));
    }

    pub fn pop_htlc(&mut self) -> (Htlc, Timestamp) {
        assert!(!self.all_slots_free());
        let raw_tuple = self.slots.pop();
        if let Some(raw_tuple) = raw_tuple {
            (raw_tuple.0, raw_tuple.1.0)
        } else {
            panic!("Got an empty HTLC, must have checked that slots are not empty!");
        }
    }

    pub fn get_earliest_htlc_resolution_time(&self) -> &Timestamp {
        assert!(!self.all_slots_free());
        // .1 takes reversed timestamp from (htlc, reversed timestamp) tuple
        // .0 takes the timestamp from reversed timestamp
        &self.slots.peek().unwrap().1.0
    }

    fn ensure_free_slots(
        &mut self,
        time: &Timestamp,
        num_slots_needed: usize
    ) -> (bool, Vec<(Htlc, Timestamp)>) {
        // Comment from Python implementation:
		// # Ensure there are num_slots_needed free slots in the HTLC queue.
		// # If the queue is full, check the timestamp of the earliest in-flight HTLC.
		// # If it is in the past, pop that HTLC (so it can be resolved).
		// # Repeat until enough slots are freed up, or until the next earliest HTLC isn't outdated.
		// # In the former case, re-insert the popped HTLCs back into the queue.
		// # Return success (True / False) and the released HTLCs, if any, along with their timestamps.
        // TODO: rethink this logic?
        // May seem wasteful that we release _all_ HTLC we can when we need just one
        // This is related to potential circular routes, AFAIU
        // If we prohibit loops, we can pop and resolve one by one if needed.
        // However, routes with loops are useful in attack simulations.
        // For now, we simply translate Python code, maybe optimize later.
        let num_free_slots = self.get_num_slots_free();
        if num_free_slots >= num_slots_needed {
            return (true, Vec::<(Htlc, Timestamp)>::new());
        } else {
            let num_htlcs_to_release = num_slots_needed - num_free_slots;
            let mut released_htlcs: Vec<(Htlc, Timestamp)> = Vec::new();
            for _ in 1..=num_htlcs_to_release {
                if self.get_earliest_htlc_resolution_time() <= time {
                    // TODO: isolate the priority queue functionality into a separate module
                    released_htlcs.push(self.pop_htlc());
                } else {
                    for (htlc, timestamp) in released_htlcs {
                        self.push_htlc(timestamp, htlc);
                    }
                    return (false, Vec::<(Htlc, Timestamp)>::new());
                }
            }
            (true, released_htlcs)
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::paymentid::PaymentId as PaymentId;
    use crate::common::paymentresult::PaymentResult as PaymentResult;
    use crate::common::satoshi::Satoshi as Satoshi;
    use crate::common::fee::FeeRate as FeeRate;

    #[test]
    pub fn channelindirection_assign_fee() {
        let mut ch_in_dir = ChannelInDirection::new(
            None,
            Some(Fee::new(Satoshi(5), FeeRate(0.02))),
            Some(Fee::new(Satoshi(1), FeeRate(0.03))),
            None,
            None,
        );
        let amount = Satoshi(100);
        let success_fee = ch_in_dir.requires_fee(FeeType::Success, &amount);
        let upfront_fee = ch_in_dir.requires_fee(FeeType::Upfront, &amount);
        let total_fee = success_fee.add(&upfront_fee);
        assert_eq!(success_fee, Satoshi(4));
        assert_eq!(upfront_fee, Satoshi(7));
        assert_eq!(ch_in_dir.requires_total_fee(&amount), total_fee);
    }

    #[test]
    #[should_panic]
    pub fn channelindirection_too_many_slots() {
        let ch_in_dir = ChannelInDirection::new(
            Some(params::MAX_NUM_SLOTS + 1),
            None,
            None,
            None,
            None);
    }

    #[test]
    #[should_panic]
    pub fn channelindirection_zero_slots() {
        let ch_in_dir = ChannelInDirection::new(
            Some(0),
            None,
            None,
            None,
            None);
    }

    fn assert_queue_is_empty(ch_in_dir: &ChannelInDirection) {
        assert!(ch_in_dir.all_slots_free());
        assert!(!ch_in_dir.all_slots_busy());
        assert_eq!(ch_in_dir.get_num_slots_busy(), 0);
    }

    #[test]
    #[should_panic]
    pub fn channelindirection_no_earliest_timestamp_in_empty_queue() {
        let ch_in_dir = ChannelInDirection::new_default();
        let t = ch_in_dir.get_earliest_htlc_resolution_time();
    }

    #[test]
    pub fn channelindirection_queue_size() {
        // set up an empty channelindirection with 2 slots
        let mut ch_in_dir = ChannelInDirection::new(
            Some(2), None, None, None, None,
        );
        assert_queue_is_empty(&ch_in_dir);
        assert_eq!(ch_in_dir.get_num_slots_free(), 2);
        assert_queue_is_empty(&ch_in_dir);
        // push one HTLC
        let htlc1 = Htlc::new(
            PaymentId(String::from("htlc1")),
            Satoshi(1),
        PaymentResult::SUCCESS,
        );
        ch_in_dir.push_htlc(Timestamp(10), htlc1);
        assert!(!ch_in_dir.all_slots_free());
        assert!(!ch_in_dir.all_slots_busy());
        assert_eq!(ch_in_dir.get_num_slots_busy(), 1);
        assert_eq!(ch_in_dir.get_num_slots_free(), 1);
        assert_eq!(ch_in_dir.get_earliest_htlc_resolution_time(), &Timestamp(10));
        // push another HTLC
        let htlc2 = Htlc::new(
            PaymentId(String::from("htlc2")),
            Satoshi(2),
        PaymentResult::SUCCESS,
        );
        ch_in_dir.push_htlc(Timestamp(5), htlc2);
        assert!(!ch_in_dir.all_slots_free());
        assert!(ch_in_dir.all_slots_busy());
        assert_eq!(ch_in_dir.get_num_slots_busy(), 2);
        assert_eq!(ch_in_dir.get_num_slots_free(), 0);
        assert_eq!(ch_in_dir.get_earliest_htlc_resolution_time(), &Timestamp(5));
        // pop htlc - the lower-timestamp one is popped first
        let htlc_popped = ch_in_dir.pop_htlc();
        let (htlc, resolution_time) = htlc_popped;
        assert_eq!(htlc.get_id(), &PaymentId(String::from("htlc2")));
        assert_eq!(resolution_time, Timestamp(5));
        assert_eq!(ch_in_dir.get_earliest_htlc_resolution_time(), &Timestamp(10));
        // pop htlc - the other one is popped second
        let htlc_popped = ch_in_dir.pop_htlc();
        let (htlc, resolution_time) = htlc_popped;
        assert_eq!(htlc.get_id(), &PaymentId(String::from("htlc1")));
        assert_eq!(resolution_time, Timestamp(10));
        // the queue is empty now
        assert_queue_is_empty(&ch_in_dir);
        assert_eq!(ch_in_dir.get_num_slots_free(), 2);
    }

    #[test]
    fn channelindirection_ensure_free_slots() {
        let mut ch_in_dir = ChannelInDirection::new(
            Some(4), None, None, None, None,
        );
        let htlc1 = Htlc::new(
            PaymentId(String::from("htlc1")),
            Satoshi(100),
        PaymentResult::SUCCESS,
        );
        let htlc2 = Htlc::new(
            PaymentId(String::from("htlc2")),
            Satoshi(200),
        PaymentResult::SUCCESS,
        );
        let htlc3 = Htlc::new(
            PaymentId(String::from("htlc3")),
            Satoshi(300),
        PaymentResult::SUCCESS,
        );
        ch_in_dir.push_htlc(Timestamp(5), htlc1);
        ch_in_dir.push_htlc(Timestamp(10), htlc2);
        ch_in_dir.push_htlc(Timestamp(15), htlc3);
        // at time 3, we can ensure one slot, no HTLCs are released
        let (success, released_htlcs) = ch_in_dir.ensure_free_slots(
            &Timestamp(3), 1);
        assert_eq!(success, true);
        assert_eq!(released_htlcs.len(), 0);
        // at time 5, we can ensure one slot, on HTLCs are released 
        let (success, released_htlcs) = ch_in_dir.ensure_free_slots(
            &Timestamp(5), 1);
        assert_eq!(success, true);
        assert_eq!(released_htlcs.len(), 0);
        // at time 6, we can ensure one slot, on HTLCs are released 
        let (success, released_htlcs) = ch_in_dir.ensure_free_slots(
            &Timestamp(6), 1);
        assert_eq!(success, true);
        assert_eq!(released_htlcs.len(), 0);
        // at time 5, we can ensure two slots by releasing the htlc with resolution time 5
        let (success, released_htlcs) = ch_in_dir.ensure_free_slots(
            &Timestamp(5), 2);
        assert_eq!(success, true);
        assert_eq!(released_htlcs.len(), 1);
        // now there are two free slots
        assert_eq!(ch_in_dir.get_num_slots_free(), 2);
        // at time 12, we cannot ensure 4 slots
        let (success, released_htlcs) = ch_in_dir.ensure_free_slots(
            &Timestamp(12), 4);
        assert_eq!(success, false);
        assert_eq!(released_htlcs.len(), 0);
        // at time 12, we can ensure 3 slots
        let (success, released_htlcs) = ch_in_dir.ensure_free_slots(
            &Timestamp(12), 3);
        assert_eq!(success, true);
        assert_eq!(released_htlcs.len(), 1);
        assert_eq!(ch_in_dir.get_num_slots_free(), 3);

    }

}