use crate::common::satoshi::Satoshi as Satoshi;

#[derive(Debug)]
pub enum FeeType {
    Upfront,
    Success,
}

#[derive(Debug, Clone)]
pub struct FeeRate(pub f64);

#[derive(Debug, Clone)]
pub struct Fee {
    base_fee: Satoshi,
    fee_rate: FeeRate,
}
impl Default for Fee {
    fn default() -> Fee {
        Fee::new(Satoshi(0), FeeRate(0.0))
    }
}
impl Fee {
    pub fn apply(&self, amount: &Satoshi) -> Satoshi {
        amount.mul(self.fee_rate.0).add(&self.base_fee)
    }
    pub fn new(base_fee: Satoshi, fee_rate: FeeRate) -> Self {
        // Can a fee rate be negative?
        // Well, it can certainly be zero.
        // We haven't thought much about implications of negative fees,
        // although that could be an interesting research question.
        // To ensure correctness w.r.t. existing code,
        // we could have used the following assert here,
        // but does equality comparison work with floats?..
        // Let's leave it at that: rates may be negative,
        // but negative rates are not (yet) used in simulations.
        //assert!(fee_rate.0 >= 0.0);
        Fee {
            base_fee: base_fee,
            fee_rate: fee_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_application() {
        let amount = Satoshi(100);
        let fee = Fee::new(Satoshi(2), FeeRate(0.01));
        assert_eq!(fee.apply(&amount), Satoshi(3));
    }
}