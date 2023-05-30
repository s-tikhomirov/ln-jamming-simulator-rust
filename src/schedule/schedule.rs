use priority_queue::PriorityQueue;
use std::cmp::Reverse;

use crate::common::timestamp::Timestamp;
use crate::common::satoshi::Satoshi as Satoshi;

use super::event::Event;

#[derive(Debug)]
pub struct Schedule {
    end_time: Timestamp,
    schedule: PriorityQueue<Event, Reverse<Timestamp>>,
}

impl Schedule {
    pub fn new(end_time: Timestamp) -> Self {
        Self {
            end_time: end_time,
            schedule: PriorityQueue::new(),
        }
    }
    pub fn get_num_events(&self) -> usize {
        self.schedule.len()
    }
    pub fn get_event(&mut self) -> (Option<Timestamp>, Option<Event>) {
        let maybe_tuple = self.schedule.pop();
        match maybe_tuple {
            Some(maybe_tuple) => {
                let event = maybe_tuple.0;
                let time = maybe_tuple.1.0;
                (Some(time), Some(event))
            }
            None => (None, None)
        }
    }
    // pub fn get_all_events(&mut self) -> Vec<(Timestamp, Event)> {
    //     // TODO
    // }
    pub fn no_more_events(&self) -> bool {
        self.schedule.is_empty()
    }
    pub fn put_event(&mut self, event_time: Timestamp, event: Event, current_time: Timestamp) {
        // we can only put events for the future
        assert!(current_time < event_time);
        // we cannot put events after the schedule end time
        assert!(event_time <= self.end_time);
        self.schedule.push(event, Reverse(event_time));
    }
}

// TODO: implement HonestSchedule and JammingSchedule as
// implementations of Schedule trait (?)

#[cfg(test)]
mod tests {
    use crate::common::{nodeid::NodeId, duration::Duration, paymentresult::PaymentResult};

    use super::*;

    #[test]
    pub fn schedule_push_pop() {
        let mut sch = Schedule::new(Timestamp(10));
        assert!(sch.no_more_events());
        let event1 = Event::new(
            NodeId(String::from("Alice")),
            NodeId(String::from("Bob")),
            Satoshi(100),
            Duration(5),
            PaymentResult::SUCCESS,
            None,
        );
        let event2 = Event::new(
            NodeId(String::from("Alice")),
            NodeId(String::from("Bob")),
            Satoshi(200),
            Duration(5),
            PaymentResult::SUCCESS,
            None,
        );
        // resolves at time 11 (that's OK)
        sch.put_event(Timestamp(6), event1, Timestamp(0));
        assert!(!sch.no_more_events());
        // resolves at time 8
        sch.put_event(Timestamp(3), event2, Timestamp(0));
        assert!(!sch.no_more_events());
        // pop earlier event
        let (time, event) = sch.get_event();
        assert!(time.is_some() && event.is_some());
        let time = time.unwrap();
        assert_eq!(time, Timestamp(3));
        // pop later event
        let (time, event) = sch.get_event();
        assert!(time.is_some() && event.is_some());
        let time = time.unwrap();
        assert_eq!(time, Timestamp(6));
    }
    
    fn schedule_push_post_end_time_not_allowed() {
        let mut sch = Schedule::new(Timestamp(10));
        let event = Event::new(
            NodeId(String::from("Alice")),
            NodeId(String::from("Bob")),
            Satoshi(100),
            Duration(5),
            PaymentResult::SUCCESS,
            None,
        );
        // it's OK for an event to _resolve_ after schedule's end time
        // it's not OK to _start_ after schedule's end time
        sch.put_event(Timestamp(11), event, Timestamp(5));
    }
    
}