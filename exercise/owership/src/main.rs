#![allow(unused_mut)]
fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    inspect(&arg);
    fn inspect(s:&String) {
        if s.ends_with("s") {
             println!("{} is plural", s);
        } else {
             println!("{} is singular", s);
        }
       
       
    }
   let mut s1 = String::from("hello world");
   let s2 = s1.clone();
   println!("{}", s1);
   do_stuff(&s1);
   do_stuff1(&mut s1)
}
fn do_stuff(s: &String) {
    println!("{}", s);
}
fn do_stuff1(s: &mut String) {
    s.insert_str(0, "fuck ");
    println!("{}", s);
}
