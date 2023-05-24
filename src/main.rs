use ln_jamming_simulator::channel;
use ln_jamming_simulator::satoshi::Satoshi as Satoshi;

fn main() {
    println!("Welcome to LN jamming simulator!");
    channel::test_channel();
    println!("Satoshi might be useful here too: I have {:?}", Satoshi(100));
}