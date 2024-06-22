const STARTING_MISSILES: u32 = 8;
const READY_AMOUNT: u32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    
    println!("Firing {} of my {} missiles...", READY_AMOUNT, missiles);
    missiles = missiles - READY_AMOUNT;
    println!("{} missiles left", missiles);
}
