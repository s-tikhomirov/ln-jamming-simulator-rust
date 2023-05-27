use crate::common::paymentid::PaymentId as PaymentId;
use crate::common::satoshi::Satoshi as Satoshi;
use crate::common::paymentresult::PaymentResult as PaymentResult;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Htlc {
    payment_id: PaymentId,
    success_fee_satoshi: Satoshi,
    desired_result: PaymentResult,
}
impl Htlc {
    // TODO: generate random payment Id if not given
    pub fn new(payment_id: PaymentId, success_fee_satoshi: Satoshi, desired_result: PaymentResult) -> Htlc {
        Htlc {
            payment_id,
            success_fee_satoshi,
            desired_result,
        }
    }
    pub fn get_id(&self) -> &PaymentId {
        &self.payment_id
    }
}