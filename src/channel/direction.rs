use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Direction {
    Alph,
    NonAlph,
}
impl Direction {
    pub fn new(a: &str, b: &str) -> Direction {
        match a.cmp(b) {
            Ordering::Less => Direction::Alph,
            Ordering::Greater => Direction::NonAlph,
            Ordering::Equal => {
                panic!("Node IDs must be different to calculate direction, got {} and {}", a, b);
            }
        }
    }
}