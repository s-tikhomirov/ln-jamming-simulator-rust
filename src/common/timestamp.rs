use core::time;

use crate::common::duration::Duration as Duration;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn add(&self, duration: &Duration) -> Timestamp {
        Timestamp(self.0 + duration.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn timestamp_add_duration() {
        let ts1 = Timestamp(2);
        let duration = Duration(3);
        let ts2 = &ts1.add(&duration);
        assert_eq!(ts2, &Timestamp(5));
    }
}