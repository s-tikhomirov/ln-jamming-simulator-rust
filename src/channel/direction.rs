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
    pub fn get_direction(a: &str, b: &str) -> Direction {
        // can this be part of Direction enum, like a constructor?
        match a.cmp(b) {
            Ordering::Less => Direction::Alph,
            Ordering::Greater => Direction::NonAlph,
            Ordering::Equal => Direction::Alph, // FIXME: this sould be an assert or something
        }
    }
}