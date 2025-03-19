/*
    // Notes taken from -> Rust Atomics and Locks: Low-Level Concurrency in Practice (Mara Bos)

Atomics
- Chapter 1
    - Scoped threads cant outlive the scope of the closure we pass to that function
    
    - When interior mutable types are involved, it's more accurate to use the terms: shared and exclusive
        - a shared reference (&T) can be copied and shared with others
        - an exclusive reference (&mut T) guarantees it’s the only exclusive borrowing of that T
        - Interior mutability only bends the rules for shared borrowing
        - While Cell and RefCell can be useful, they are useless when using multiple threads
        
    - RwLock
        - Concurrent version of RefCell
        - An RwLock<T> holds a T and tracks any outstanding borrows, blocks on conflicting borrows
        - Requires traits: Send, Sync
        - Three states: unlocked, single writer lock (exclusive), and locked by # of readers (for shared access)
        - std::sync::RwLock<T> type
            - RwLockReadGuard: Deref
            - RwLockWriteGuard: Deref, DerefMut
    
    - Atomic Types: represent the concurrent version of a Cell
        - No generics, only concrete types in library
    
    - Mutex
        - No unlock method, only scope-bounded once lock() called
        - Requires only the Send trait
        - If a lock is held while it's panics, the lock is poisoned... future lock() calls return an Err
        - Locks are not dropped automatically until the end of a compound statement (if let/while let), simple boolean statements do not apply (if .lock().unwrap() == Some(1) {})
            - Cause of confusion/deadlock

- Chapter 2
    - Atomics (std::sync::atomic)
        - Allow modification through a shared reference (ex. &AtomicU8)
        - All have the same interface/protocol: ex. storing and loading methods
        - Every atomic operation takes an arg of type std::sync::atomic::Ordering

    - LOOK at the .fetch_update() method for the the compare-and-exchnage loop pattern

- Chapter 3
    - 

- Chapter 4
    -

- Chapter 9
    - 

- Chapter 10
    - Focus on: RCU, Parking Lot-Based Locks

*/

use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
////////////////////////////////
// Scoped Threads
let numbers = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| {
        println!("length: {}", numbers.len());
    });
    s.spawn(|| {
        for n in &numbers {
            println!("{n}");
        }
    });
});
////////////////////////////////
// Manually dropping the guard before scope-bounded drop (DerefMut)
fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard); // New: drop the guard before sleeping!
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}
/////////////////////////////////
mpl AtomicI32 {
    pub fn load(&self, ordering: Ordering) -> i32;
    pub fn store(&self, value: i32, ordering: Ordering);
}
////////////////////////////////
use std::sync::atomic::AtomicUsize;

fn main() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Assuming this takes some time.
                num_done.store(i + 1, Relaxed);
                num_done.fetch_add(0, Relaxed); // Couldve used this with a 1 instead of the store op
                main_thread.unpark(); // Wake up the main thread.
            }
        });

        // The main thread shows status updates.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
/////////////////////////////////////
// Look at the .fetch_update() to solve this compare-and-exchange loop pattern
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}
//////////////////////////////////////
// Look at std::sync::Once and std::sync::OnceLock for lazy inits that take a lot of time on startup
fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => key, 
        }
    } else {
        key
    }
}
///////////////////////////////////////
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Bank {
    accounts: RwLock<HashMap<String, f64>>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: RwLock::new(HashMap::new()),
        }
    }

    fn deposit(&self, account: &str, amount: f64) {
        let mut accounts = self.accounts.write().unwrap();
        let balance = accounts.entry(account.to_string()).or_insert(0.0);
        *balance += amount;
    }

    fn withdraw(&self, account: &str, amount: f64) {
        let mut accounts = self.accounts.write().unwrap();
        if let Some(balance) = accounts.get_mut(account) {
            if *balance >= amount {
                *balance -= amount;
            } else {
                println!("Insufficient funds for account: {}", account);
            }
        }
    }

    fn check_balance(&self, account: &str) -> f64 {
        let accounts = self.accounts.read().unwrap();
        *accounts.get(account).unwrap_or(&0.0)
    }
}

fn main() {
    let bank = Arc::new(Bank::new());
    let mut handles = vec![];

    for i in 0..5 {
        let bank_clone = Arc::clone(&bank);
        let handle = thread::spawn(move || {
            let account = format!("Account{}", i);
            bank_clone.deposit(&account, 100.0);
            thread::sleep(Duration::from_millis(100)); // Simulate other work
            bank_clone.withdraw(&account, 30.0);
            let balance = bank_clone.check_balance(&account);
            println!("{} - Balance: {:.2}", account, balance);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
