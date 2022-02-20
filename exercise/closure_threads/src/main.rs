
#![allow(dead_code,unused_imports,unused_variables)]

use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum (v:Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child Thread: just about finished");
    v.iter()
        .filter(|&x| x %2 == 0)
        .map(|x| x * x)
        .sum()

}
fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let handle = thread::spawn(move|| expensive_sum(my_vector));
    
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("main thread: {}", letter);
        pause_ms(200);
    }
    let sum = handle.join().unwrap();
    println!("main thread: sum is {}", sum);

    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(500);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });
    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }
    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!("Main thread: Exiting.")
}

fn pause_ms(ms:u64) {
    thread::sleep(Duration::from_millis(ms));
}
