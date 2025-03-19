/*
Concurrency
    - Rustaceans view concurrent as: concurrency and/or parallel
    
    - To create a thread, use the thread::spawn function
        - join to wait
    
    - Ways to share information between threads
        1. Message passing
        2. Shared state

    - Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer called MutexGuard, 
        wrapped in a LockResult that we handled with the call to unwrap. 
        The MutexGuard smart pointer implements Deref to point at our inner data; 
        the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope, 
        which happens at the end of the inner scope.

    - Two major traits
        1. Sync: allowing access from multiple threads
            - In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread. 
        2. Send: allowing transference of ownership between threads
*/
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
///////////////////////////////////////////////////////////////////
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx)); // Wrap tx in Arc and Mutex

    // Create a vector of transmitters, each cloned from the original `tx`
    let tx_clones = vec![Arc::clone(&tx), Arc::clone(&tx)];

    // Spawn multiple threads, each using a different transmitter
    for tx in tx_clones {
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                // Lock the Mutex to safely send the message
                let tx = tx.lock().unwrap();
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    // Receive messages in the main thread and print them
    for received in rx {
        println!("Got: {received}");
    }
}
