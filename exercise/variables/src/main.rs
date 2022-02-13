fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    // let
    let mut a = 1;
    let (bunnies,carrots) = (1,2);
    a = 2;
    // const uppercase the word with underscore
    // - type annotation is required
    // - define a int the type is i64
    const QQ_STR : i64= 1;
    println!("{},{}",QQ_STR,a);
    a = a-bunnies;
    println!("{}",a);

    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
    
}

// notes;
// - Rust is a statically typed language.
// - variables are immutable by default in Rust, it's because Rust concerns safety, concurrency, speed.
