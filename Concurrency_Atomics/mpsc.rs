/*
- Experimental: mpmc

mpsc
    - Multi-producer, single-consumer FIFO queue
    
    - Three types for channels
        1. Sender
        2. SyncSender
        3. Receiver

    - Two channel flavors
        1. Async: channel function returns (Sender, Receiver)
        2. Sync: sync_channel function returns (SyncSender, Receiver)
            - Storage for pending message is a pre-allocated buffer of a fixed size
            - All send will be blocked until enough space is available
*/

use std::thread;
use std::sync::mpsc::channel;

// Create a shared channel that can be sent along from many threads
// where tx is the sending half (tx for transmission), and rx is the receiving
// half (rx for receiving).
let (tx, rx) = channel();
for i in 0..10 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}

for _ in 0..10 {
    let j = rx.recv().unwrap();
    assert!(0 <= j && j < 10);
}