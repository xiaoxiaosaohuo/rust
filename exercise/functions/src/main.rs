#[warn(unused_variables)]
use rand::Rng;
mod math;
fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    println!("The area is {}", area_of(width, height));
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);

    println!("32 + 67 = {}", math::sum(32, 67));
}

fn area_of(x:i32,y:i32) -> i32{
    // return x * y;
    x*y
}
