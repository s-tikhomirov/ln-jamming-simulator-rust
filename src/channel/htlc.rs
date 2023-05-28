use crate::common::satoshi::Satoshi as Satoshi;
use crate::common::paymentresult::PaymentResult as PaymentResult;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Htlc {
    success_fee_satoshi: Satoshi,
    desired_result: PaymentResult,
}
impl Htlc {
    // TODO: generate random payment Id if not given
    pub fn new(success_fee_satoshi: Satoshi, desired_result: PaymentResult) -> Htlc {
        Htlc {
            success_fee_satoshi,
            desired_result,
        }
    }
}