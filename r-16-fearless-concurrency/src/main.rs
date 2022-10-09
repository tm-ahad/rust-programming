use std::sync::{Arc, mpsc, Mutex};
use std::thread;

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

    //message passing between threads

    println!("==========================");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let msg = rx.recv().unwrap();

    println!("{:?} received from spawned thread", msg);

    //sharing state
    println!("==========================");

    let v: Vec<u32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    let counter = Arc::new(Mutex::new(v[0]));
    let mut handles = vec![];

    for val in v {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num = val;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
