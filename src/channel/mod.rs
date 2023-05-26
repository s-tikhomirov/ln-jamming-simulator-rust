use std::collections::HashMap;

mod channelindirection;
mod direction;

use crate::common::satoshi::Satoshi as Satoshi;
use crate::common::fee::Fee as Fee;
use crate::common::fee::FeeRate as FeeRate;
use crate::common::fee::FeeType as FeeType;

use direction::Direction as Direction;
use channelindirection::ChannelInDirection as ChannelInDirection;

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


mod tests {
    use super::*;

    #[test]
    fn channel_dummy_test() {
        // let ch = Channel {
        //     capacity: Satoshi(1000),
        //     cid: String::from("cid0"),
        //     channel_in_direction: HashMap::from([
        //         (Direction::Alph, Some(ch_in_dir)),
        //         (Direction::NonAlph, None),
        //     ])
        // };
        // TODO: continue
        assert!(true);
    }
}