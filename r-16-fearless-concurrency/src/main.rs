use std::thread;
use std::time::Duration;

fn main() {
    //single thread

    thread::spawn(|| {
        println!("stuff 1 from spawned thread");
        println!("stuff 2 from spawned thread");
    });

    println!("stuff 1 from main thread");
    println!("stuff 2 from main thread");

    println!("==========================");

    //managing multiple thread with vectors
    let mut threads = vec![];

    for _ in 0..10
    {
        let thread = thread::spawn(||
            {
                println!("Hi from one of spawn threads!");
            });

        threads.push(thread);
    }

    //joining threads

    for t in threads
    {
        t.join().unwrap();
    }

    println!("Hi from main thread!");
}
