use crate::common::satoshi::Satoshi as Satoshi;
use crate::common::fee::{Fee as Fee, FeeType as FeeType};

#[derive(Debug)]
pub struct ChannelInDirection {
    // TODO: implement slots as a priority queue
    pub num_slots: u16,
    pub upfront_fee: Fee,
    pub success_fee: Fee,
    pub deliberately_fail_prob: f64,    // can I define a sub-type of float for probabilities to check 0<=x<=1?
    pub spoofing_error_type: Option<crate::channel::ErrorType>,
}

// https://stackoverflow.com/a/19653453/5752262
impl Default for ChannelInDirection {
    fn default() -> Self {
        Self::new(
            crate::channel::params::NUM_SLOTS,
            Fee::default(),
            Fee::default(),
            0.0,
            None,
        )
    }
}

impl ChannelInDirection {

    pub fn new(
        num_slots: u16,
        upfront_fee: Fee,
        success_fee: Fee,
        deliberately_fail_prob: f64,
        spoofing_error_type: Option<crate::channel::ErrorType>
    ) -> Self {
        ChannelInDirection {
            num_slots,
            upfront_fee,
            success_fee,
            deliberately_fail_prob,
            spoofing_error_type,
            }
    }

    pub fn set_fee(&mut self, fee_type: FeeType, fee: Fee) {
        match fee_type {
            FeeType::Success => self.success_fee = fee,
            FeeType::Upfront => self.upfront_fee = fee,
        }
    }

    pub fn requires_fee(&self, fee_type: Option<FeeType>, amount: &Satoshi) -> Satoshi {
        match fee_type {
            Some(FeeType::Success) => self.success_fee.apply(&amount),
            Some(FeeType::Upfront) => self.upfront_fee.apply(&amount),
            None => {
                let upfront_fee = self.upfront_fee.apply(&amount);
                let success_fee = self.success_fee.apply(&amount);
                success_fee.add(&upfront_fee)
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::satoshi::Satoshi as Satoshi;
    use crate::common::fee::FeeRate as FeeRate;

    #[test]
    pub fn channelindirection_assign_fee() {
        let mut ch_in_dir = ChannelInDirection {
            num_slots: 500,
            success_fee: Fee::new(Satoshi(1), FeeRate(0.03)),
            ..Default::default()
        };
        assert_eq!(ch_in_dir.requires_fee(None, &Satoshi(100)), Satoshi(4));
        ch_in_dir.set_fee(
            FeeType::Success, 
            Fee::new(Satoshi(5), FeeRate(0.02)),
        );
        assert_eq!(ch_in_dir.requires_fee(None, &Satoshi(100)), Satoshi(7));
    }
}