#[derive(Debug)]
pub struct Satoshi(pub u64);

impl Satoshi {
    pub fn add(&self, other_amount: &Satoshi) -> Satoshi {
        Satoshi(self.0 + other_amount.0)
    }
    pub fn mul (&self, coeff: f64) -> Satoshi {
        Satoshi((self.0 as f64 * coeff) as u64)
    }
}