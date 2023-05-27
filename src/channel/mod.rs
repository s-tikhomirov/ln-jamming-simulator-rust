use std::collections::HashMap;

mod channelindirection;
mod direction;
mod htlc;

use crate::common::satoshi::Satoshi as Satoshi;
use crate::common::fee::Fee as Fee;
use crate::common::fee::FeeRate as FeeRate;
use crate::common::fee::FeeType as FeeType;

use direction::Direction as Direction;
use channelindirection::ChannelInDirection as ChannelInDirection;

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