use schedule::schedule::Schedule;

use crate::common::paymentresult::PaymentResult;
use crate::common::timestamp::Timestamp;
use crate::common::satoshi::Satoshi;
use crate::common::fee::{Fee, FeeRate};

pub mod channel;
pub mod common;
pub mod payment;
pub mod schedule;

const SUCCESS_PROBABILITY: f64 = 1.0;
const SIMULATION_END_TIME: Timestamp = Timestamp(100);
const INITIAL_BALANCE: Satoshi = Satoshi(1_000_000);

pub fn simulate() {
    println!("Entering simulation");
    // create new schedule
    // populate the schedule
    // set initiali balances for Alice, Bob, and fees paid
    // loop through schedule: pop next event and apply
    // assert that final balances sum up
    // print the results
}
