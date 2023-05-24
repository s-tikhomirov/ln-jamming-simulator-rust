use std::collections::HashMap;

mod channelindirection;
mod direction;

use crate::satoshi::Satoshi as Satoshi;
use direction::Direction as Direction;
use channelindirection::ChannelInDirection as ChannelInDirection;
use crate::fee::Fee as Fee;
use crate::fee::FeeRate as FeeRate;
use crate::fee::FeeType as FeeType;

pub mod params {
    // ProtocolParams
    pub const DUST_LIMIT: u16 = 354;
    pub const NUM_SLOTS: u16 = 483;
    pub const MAX_ROUTE_LENGTH: u16 = 20;

    // FeeParams
    pub const SUCCESS_BASE: u64 = 1;
    pub const SUCCESS_RATE: f64 = 5.0 / (1000 * 1000) as f64;   // is it normal to do this type casting?
    // why doesn't it understand that I have f64 on the left?

}

////////////////////// ENUMTYPES.PY ////////////////

#[derive(Debug)]
pub enum ErrorType {
    LowBalance,
    NoSlots,
    LowFee,
    FailedDeliberately,
}

//////////////////// CHANNEL.PY //////////////////////////


#[derive(Debug)]
pub struct Channel {
    capacity: Satoshi,
    cid: String,
    channel_in_direction: HashMap<Direction, Option<ChannelInDirection>>,
}
pub fn test_channel() {

    //println!("Welcome to LN jamming simulator in Rust!");
    let id_a = "a";
    let id_b = "b";
    let dir1 = Direction::get_direction(id_a, id_b);
    let dir2 = Direction::get_direction(id_b, id_a);
    let are_equal = dir1 == dir2;
    println!("{:?} {:?} {}",dir1, dir2, are_equal);

    let mut ch_in_dir = channelindirection::ChannelInDirection {
        num_slots: 500,
        success_fee: Fee::new(Satoshi(1), FeeRate(0.03)),
        ..Default::default()
    };
    println!("{:?}", ch_in_dir);
    let test_amount = Satoshi(100);
    println!(
        "Payment of {:?} requires fee {:?}",
        test_amount,
        ch_in_dir.requires_fee(None, &test_amount));
    ch_in_dir.set_fee(
        FeeType::Success, 
        Fee::new(Satoshi(5), FeeRate(0.02)),
    );
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
}

