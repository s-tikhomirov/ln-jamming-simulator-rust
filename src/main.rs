
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

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
#[derive(Eq)]
#[derive(Hash)]
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
struct Satoshi(u64);

impl Satoshi {
    fn add(&self, other_amount: &Satoshi) -> Satoshi {
        Satoshi(self.0 + other_amount.0)
    }
    fn mul(&self, fee_rate: &FeeRate) -> Satoshi {
        Satoshi((self.0 as f64 * fee_rate.0) as u64)
    }
}

#[derive(Debug)]
struct FeeRate(f64);


#[derive(Debug)]
struct Fee {
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


////////////////// CHANNELDIRECTION.PY ////////////

#[derive(Debug)]
struct ChannelInDirection {
    num_slots: u16,
    upfront_fee: Fee,
    success_fee: Fee,
    deliberately_fail_prob: f64,    // can I define a sub-type of float for probabilities to check 0<=x<=1?
    spoofing_error_type: Option<ErrorType>,
}

// https://stackoverflow.com/a/19653453/5752262
impl Default for ChannelInDirection {
    fn default() -> ChannelInDirection {
        ChannelInDirection {
            num_slots: NUM_SLOTS,
            upfront_fee: Fee::default(),
            success_fee: Fee::default(),
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
    fn generic_fee_function(fee: &Fee, amount: &Satoshi) -> Satoshi {
        amount.mul(&fee.fee_rate).add(&fee.base_fee)
    }

    fn set_fee(&mut self, fee_type: FeeType, fee: Fee) {
        // should I conver ints to Amounts and floats to FeeRates inside or outside the function?
        // FIXME: in Python, I also assign fee_function here as lambda
        match fee_type {
            FeeType::Success => self.success_fee = fee,
            FeeType::Upfront => self.upfront_fee = fee,
        }
    }

    fn requires_fee(&self, fee_type: Option<FeeType>, amount: &Satoshi) -> Satoshi {
        match fee_type {
            Some(FeeType::Success) => {
                ChannelInDirection::generic_fee_function(&self.success_fee, &amount)
            }
            Some(FeeType::Upfront) => {
                ChannelInDirection::generic_fee_function(&self.upfront_fee, &amount)
            }
            None => {
                let upfront_fee = ChannelInDirection::generic_fee_function(&self.upfront_fee, &amount);
                let success_fee = ChannelInDirection::generic_fee_function(&self.success_fee, &amount);
                success_fee.add(&upfront_fee)
            }

        }
    }
}


//////////////////// CHANNEL.PY //////////////////////////

#[derive(Debug)]
struct Channel {
    capacity: Satoshi,
    cid: String,
    channel_in_direction: HashMap<Direction, Option<ChannelInDirection>>,
}




fn main() {
    //println!("Welcome to LN jamming simulator in Rust!");
    let id_a = "a";
    let id_b = "b";
    let dir1 = get_direction(id_a, id_b);
    let dir2 = get_direction(id_b, id_a);
    let are_equal = dir1 == dir2;
    println!("{:?} {:?} {}",dir1, dir2, are_equal);

    let mut ch_in_dir = ChannelInDirection {
        num_slots: 500,
        success_fee: Fee { base_fee: Satoshi(1), fee_rate: FeeRate(0.03) },
        ..Default::default()
    };
    println!("{:?}", ch_in_dir);
    let test_amount = Satoshi(100);
    println!(
        "Payment of {:?} requires fee {:?}",
        test_amount,
        ch_in_dir.requires_fee(None, &test_amount));
    ch_in_dir.set_fee(FeeType::Success, Fee { base_fee: Satoshi(5), fee_rate: FeeRate(0.02) });
    println!(
        "Payment of {:?} requires fee {:?}",
        test_amount,
        ch_in_dir.requires_fee(None, &test_amount));
    
    let ch = Channel {
        capacity: Satoshi(1000),
        cid: String::from("cid0"),
        channel_in_direction: HashMap::from([
            (Direction::Alph, Some(ch_in_dir)),
            (Direction::NonAlph, None),
        ])
    };

    //println!("{:#?}", ch);
}