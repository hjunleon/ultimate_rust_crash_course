
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let not_used = 0;
    STARTING_MISSILES = 2;
    // let missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;
    let (missiles, ready): (i32,i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    // let missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
