use crate::common::{
    nodeid::NodeId,
    fee::Fee,
    paymentresult::PaymentResult,
    duration::Duration,
    satoshi::Satoshi};

#[derive(Debug, Clone)]
pub struct Payment{
    pub upfront_fee_satoshi: Satoshi,
    pub success_fee_satoshi: Satoshi,
    pub desired_result: PaymentResult,
    pub processing_delay: Duration,
    pub body: Satoshi,
    downstream_node: Option<NodeId>,
    downstream_payment: Option<Box<Payment>>,
}

// compared to Python implementation, we get rid of
// "provide exactly one of (channelindirection, fee functions)" thing
// Let's simply provide fee functions as taken from channelindirection by the caller

impl Payment {
    pub fn new(
        // TODO: use references? Requires lifetimes
        upfront_fee: Fee,
        success_fee: Fee,
        desired_result: Option<PaymentResult>,
        processing_delay: Option<Duration>,
        body: Option<Satoshi>,
        downstream_node: Option<NodeId>,
        downstream_payment: Option<Box<Payment>>,
    ) -> Self {
        // for last hop, we supply values; ther is no downstream payment
        let is_last_hop =
            desired_result.is_some() &&
            processing_delay.is_some() &&
            body.is_some() &&
            downstream_node.is_none() &&
            downstream_payment.is_none();
        // for the non-last hop, values are calculated from the downstream payment,
        // which must be provided alongside the downstream node ID
        let is_not_last_hop =
            desired_result.is_none() &&
            processing_delay.is_none() &&
            body.is_none() &&
            downstream_node.is_some() &&
            downstream_payment.is_some();
        // make sure we don't mix those two cases
        assert!(is_last_hop ^ is_not_last_hop);
        if is_last_hop {
            Self {
                    upfront_fee_satoshi: upfront_fee.apply(&body.unwrap()),
                    success_fee_satoshi: Satoshi(0),
                    desired_result: desired_result.unwrap(),
                    processing_delay: processing_delay.unwrap(),
                    body: body.unwrap(),
                    downstream_node,
                    downstream_payment,
                }
        } else {
            let dp = downstream_payment.unwrap();
            let dn = downstream_node.unwrap();
            let amount = &dp.body.add(
                &success_fee.apply(&dp.body)
            );
            Self {
                upfront_fee_satoshi: upfront_fee.apply(&amount).add(&dp.upfront_fee_satoshi),
                success_fee_satoshi: (
                    success_fee.apply(
                        &dp.body.add(&dp.success_fee_satoshi)
                    ).add( 
                    &dp.success_fee_satoshi)
                ),
                desired_result: dp.desired_result.clone(),
                processing_delay: dp.processing_delay.clone(),
                body: dp.get_amount(),
                downstream_node: Some(dn),
                downstream_payment: Some(dp.clone()),
            }
        }
    }
    pub fn get_amount(&self) -> Satoshi {
        self.body.add(&self.success_fee_satoshi)
    }
}


#[cfg(test)]
mod tests {
    use crate::common::fee::FeeRate;

    use super::*;

    #[test]
    fn payment_simple() {
        let example_upfront_fee = Fee::new(Satoshi(2), FeeRate(0.02));
        let example_success_fee = Fee::new(Satoshi(5), FeeRate(0.05));
        let p_cd = Payment::new(
            // cloning as a temporary measure while I figure out lifetimes
            example_upfront_fee.clone(),
            example_success_fee.clone(),
            Some(PaymentResult::SUCCESS),
            Some(Duration(1)),
            Some(Satoshi(100)),
            None,
            None,
        );
        let p_bc = Payment::new(
            example_upfront_fee.clone(),
            example_success_fee.clone(),
            None,
            None,
            None,
            Some(NodeId(String::from("Charlie"))),
            Some(Box::new(p_cd)),
        );
        let p_ab = Payment::new(
            example_upfront_fee.clone(),
            example_success_fee.clone(),
            None,
            None,
            None,
            Some(NodeId(String::from("Bob"))),
            Some(Box::new(p_bc)),
        );
        assert_eq!(p_ab.body, Satoshi(110));
        // Python uses different rounding method (banker's rounding) by default
        // that't why the numbers in Python implementation are different
        // TODO: implement Satoshi as Decimal with the banker's rounding?
        // https://docs.rs/rust_decimal/latest/rust_decimal/index.html
        assert_eq!(p_ab.success_fee_satoshi, Satoshi(20));
        assert_eq!(p_ab.upfront_fee_satoshi, Satoshi(12));
        assert_eq!(p_ab.downstream_node, Some(NodeId(String::from("Bob"))));
        let p_bc = p_ab.downstream_payment.unwrap();
        assert_eq!(p_bc.body, Satoshi(100));
        assert_eq!(p_bc.success_fee_satoshi, Satoshi(10));
        assert_eq!(p_bc.upfront_fee_satoshi, Satoshi(8));
        assert_eq!(p_bc.downstream_node, Some(NodeId(String::from("Charlie"))));
        let p_cd = p_bc.downstream_payment.unwrap();
        assert_eq!(p_cd.body, Satoshi(100));
        assert_eq!(p_cd.success_fee_satoshi, Satoshi(0));
        assert_eq!(p_cd.upfront_fee_satoshi, Satoshi(4));
        assert_eq!(p_cd.downstream_node, None);
    }

}