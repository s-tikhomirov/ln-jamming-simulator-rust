
use std::cmp::Ordering;


// PARAMS.PY

// TODO: add payment flow params
// TODO: can we group constants into structs?

// ProtocolParams
const DUST_LIMIT: u16 = 354;
const NUM_SLOTS: u16 = 483;
const MAX_ROUTE_LENGTH: u16 = 20;

// FeeParams
const SUCCESS_BASE: u64 = 1;
const SUCCESS_RATE: f64 = 5.0 / (1000 * 1000) as f64;   // is it normal to do this type casting?
// why doesn't it understand that I have f64 on the left?

/////////////////// DIRECTION.PY /////////////////
#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    Alph,
    NonAlph,
}

fn get_direction(a: &str, b: &str) -> Direction {
    // can this be part of Direction enum, like a constructor?
    match a.cmp(b) {
        Ordering::Less => Direction::Alph,
        Ordering::Greater => Direction::NonAlph,
        Ordering::Equal => Direction::Alph, // FIXME: this sould be an assert or something
    }
}

////////////////////// ENUMTYPES.PY ////////////////

#[derive(Debug)]
enum ErrorType {
    LowBalance,
    NoSlots,
    LowFee,
    FailedDeliberately,
}

#[derive(Debug)]
enum FeeType {
    Upfront,
    Success,
}

// this is something new (wasn't in Python implementation)
// let's use wrapper structs to constraint primitive types used for different purposes
// so we never use a fee rate as a fee amount or something like this

#[derive(Debug)]
struct Amount(u64);

impl Amount {
    fn add(&self, other_amount: Amount) -> Amount {
        Amount(self.0 + other_amount.0)
    }
    fn mul(&self, fee_rate: &FeeRate) -> Amount {
        Amount((self.0 as f64 * fee_rate.0) as u64)
    }
}

#[derive(Debug)]
struct FeeRate(f64);

////////////////// CHANNELDIRECTION.PY ////////////

#[derive(Debug)]
struct ChannelInDirection {
    num_slots: u16,
    upfront_base_fee: Amount,
    upfront_fee_rate: FeeRate,
    success_base_fee: Amount,
    success_fee_rate: FeeRate,
    deliberately_fail_prob: f64,    // can I define a sub-type of float for probabilities to check 0<=x<=1?
    spoofing_error_type: Option<ErrorType>,
}

// https://stackoverflow.com/a/19653453/5752262
impl Default for ChannelInDirection {
    fn default() -> ChannelInDirection {
        ChannelInDirection {
            num_slots:          NUM_SLOTS,
            upfront_base_fee:   Amount(0),
            upfront_fee_rate:   FeeRate(0.0),
            success_base_fee:   Amount(0),
            success_fee_rate:   FeeRate(0.0),
            deliberately_fail_prob: 0.0,
            spoofing_error_type: None,
        }
    }
}

// FIXME: should we be passing ownership over amounts, as poopsed to

impl ChannelInDirection {
    // a "static method" independent of self
    // TODO: this is only useful if concrete fee functions are partial applications of this thing
    // does Rust have lambdas (closures)? Partial application?
    fn generic_fee_function(base: &Amount, rate: &FeeRate, amount: Amount) -> Amount {
        base.add(amount.mul(rate))
    }

    fn set_fee(&mut self, fee_type: FeeType, base_fee: Amount, fee_rate: FeeRate) {
        // should I conver ints to Amounts and floats to FeeRates inside or outside the function?
        // FIXME: in Python, I also assign fee_function here as lambda
        match fee_type {
            FeeType::Success => {
                self.success_base_fee = base_fee;
                self.success_fee_rate = fee_rate;
            },
            FeeType::Upfront => {
                self.upfront_base_fee = base_fee;
                self.upfront_fee_rate = fee_rate;
            }
        }
    }

    fn requires_fee(&self, fee_type: FeeType, amount: Amount) -> Amount {
        match fee_type {
            FeeType::Success => {
                ChannelInDirection::generic_fee_function(
                    &self.success_base_fee,
                    &self.success_fee_rate,
                    amount)
            }
            FeeType::Upfront => {
                ChannelInDirection::generic_fee_function(
                    &self.upfront_base_fee,
                    &self.upfront_fee_rate,
                    amount)
            }
        }
    }
}


fn main() {
    //println!("Welcome to LN jamming simulator in Rust!");
    let id_a = "a";
    let id_b = "b";
    let dir1 = get_direction(id_a, id_b);
    let dir2 = get_direction(id_b, id_a);
    let are_equal = dir1 == dir2;
    println!("{:?} {:?} {}",dir1, dir2, are_equal);

    let ch_in_dir = ChannelInDirection {
        num_slots: 500,
        success_base_fee: Amount(1),
        success_fee_rate: FeeRate(0.03),
        ..Default::default()
    };
    println!("{:?}", ch_in_dir);
    let test_amount = 100;
    let test_fee = ch_in_dir.requires_fee(FeeType::Success, Amount(test_amount));
    println!("Payment of {:?} requires fee {:?}", test_amount, test_fee)
}