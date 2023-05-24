use crate::satoshi::Satoshi as Satoshi;

#[derive(Debug)]
pub enum FeeType {
    Upfront,
    Success,
}

#[derive(Debug)]
pub struct FeeRate(pub f64);

#[derive(Debug)]
pub struct Fee {
    base_fee: Satoshi,
    fee_rate: FeeRate,
}
impl Default for Fee {
    fn default() -> Fee {
        Fee {
            base_fee: Satoshi(0),
            fee_rate: FeeRate(0.0),
        }
    }
}
impl Fee {
    pub fn apply(&self, amount: &Satoshi) -> Satoshi {
        amount.mul(self.fee_rate.0).add(&self.base_fee)
    }
    pub fn new(base_fee: Satoshi, fee_rate: FeeRate) -> Fee {
        Fee {
            base_fee: base_fee,
            fee_rate: fee_rate,
        }
    }
}