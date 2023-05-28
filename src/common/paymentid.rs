// use rand::distributions::{Alphanumeric, DistString};

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct PaymentId(pub String);

// // TODO implement random ID generation as a trait
// // that PaymentId, ChannelId, and NodeId derive
// impl PaymentId {
//     pub fn new() -> Self {
//         use rand::distributions::{Alphanumeric, DistString};
//         PaymentId(Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
//     }
// }