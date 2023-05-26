use std::cmp::Ordering;
use crate::common::nodeid::NodeId as NodeId;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Direction {
    Alph,
    NonAlph,
}
impl Direction {
    pub fn new(a: &NodeId, b: &NodeId) -> Self {
        match a.cmp(b) {
            Ordering::Less => Direction::Alph,
            Ordering::Greater => Direction::NonAlph,
            Ordering::Equal => {
                panic!("Node IDs must differ to determine direction, got {:?} and {:?}", a, b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_new() {
        let id_coingate = NodeId(
            String::from("0242a4ae0c5bef18048fbecf995094b74bfb0f7391418d71ed394784373f41e4f3")
        );
        let id_acinq = NodeId(
            String::from("03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f")
        );
        assert_eq!(Direction::new(&id_coingate, &id_acinq), Direction::Alph);
        assert_eq!(Direction::new(&id_acinq, &id_coingate), Direction::NonAlph);
    }

    #[test]
    #[should_panic]
    fn direction_new_to_self() {
        let id_acinq = NodeId(
            String::from("03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f")
        );
        Direction::new(&id_acinq, &id_acinq);
    }

}