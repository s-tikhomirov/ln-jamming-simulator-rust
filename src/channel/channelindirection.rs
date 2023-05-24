use crate::channel::Satoshi as Satoshi;
use crate::fee::Fee as Fee;
use crate::fee::FeeType as FeeType;

#[derive(Debug)]
pub struct ChannelInDirection {
    pub num_slots: u16,
    pub upfront_fee: Fee,
    pub success_fee: Fee,
    pub deliberately_fail_prob: f64,    // can I define a sub-type of float for probabilities to check 0<=x<=1?
    pub spoofing_error_type: Option<crate::channel::ErrorType>,
}

// https://stackoverflow.com/a/19653453/5752262
impl Default for ChannelInDirection {
    fn default() -> ChannelInDirection {
        ChannelInDirection {
            num_slots: crate::channel::params::NUM_SLOTS,
            upfront_fee: Fee::default(),
            success_fee: Fee::default(),
            deliberately_fail_prob: 0.0,
            spoofing_error_type: None,
        }
    }
}

impl ChannelInDirection {

    pub fn set_fee(&mut self, fee_type: FeeType, fee: Fee) {
        // should I conver ints to Amounts and floats to FeeRates inside or outside the function?
        // FIXME: in Python, I also assign fee_function here as lambda
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
