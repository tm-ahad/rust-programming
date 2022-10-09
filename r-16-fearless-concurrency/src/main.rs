use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("stuff 1 from spawned thread");
        println!("stuff 2 from spawned thread");
    });

    println!("stuff 1 from main thread");
    println!("stuff 2 from main thread");
}
