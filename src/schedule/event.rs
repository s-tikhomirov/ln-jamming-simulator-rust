use crate::common::{nodeid::NodeId, satoshi::Satoshi, duration::Duration, paymentresult::PaymentResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event {
    // do we need ID here?
    pub sender: NodeId,
    pub receiver: NodeId,
    pub amount: Satoshi,
    pub processing_delay: Duration,
    pub desired_result: PaymentResult,
    pub must_route_via_nodes: Option<Vec<NodeId>>,
}

impl Event {
    // TODO: do we need setter here?
    // it's just boilerplate code copying arguments over to struct fields
    // do we care about fields being private?
    pub fn new(
        sender: NodeId,
        receiver: NodeId,
        amount: Satoshi,
        processing_delay: Duration,
        desired_result: PaymentResult,
        must_route_via_nodes: Option<Vec<NodeId>>,
    ) -> Self {
        assert_ne!(sender, receiver);
        Self {
            sender,
            receiver,
            amount,
            processing_delay,
            desired_result,
            must_route_via_nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    pub fn event_with_same_sender_and_receiver_not_allowed() {
        let _event = Event::new(
            NodeId(String::from("Alice")),
            NodeId(String::from("Alice")),
            Satoshi(200),
            Duration(5),
            PaymentResult::SUCCESS,
            None,
        );
    }
}