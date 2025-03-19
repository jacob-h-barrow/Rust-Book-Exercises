use std::sync::Arc;

/*
Arc -> Atomically Reference Counted
    - Invoking clone on Arc produces a new Arc instance, pointing to the same allocation on the heap, and increased the reference count
    
    - Use an Atomic type (like Mutex or RwLock) to mutate what's inside Arc
        - Arc doesnt add thread safety to the underlying data
        - [IMPORTANT] Arc<T> must implement Send and Sync as long as the T implements Send and Sync

    - [IMPORTANT] downgrade method used to create non-owning Weak pointers; can be upgraded to an Arc, unless allocation has already dropped

    - 
*/
// Ex 1: Cloning references
let foo = Arc::new(vec![1.0, 2.0, 3.0]);
// The two syntaxes below are equivalent.
let a = foo.clone();
let b = Arc::clone(&foo);
// a, b, and foo are all Arcs that point to the same memory location

// Ex.2: Sharing a mutable AtomicUsize
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

let val = Arc::new(AtomicUsize::new(5));

for _ in 0..10 {
    let val = Arc::clone(&val);

    thread::spawn(move || {
        let v = val.fetch_add(1, Ordering::Relaxed);
        println!("{v:?}");
    });
}

// Ex. 3: Simple Comparison
let five = Arc::new(5);
let same_five = Arc::clone(&five);
let other_five = Arc::new(5);

assert!(Arc::ptr_eq(&five, &same_five));
assert!(!Arc::ptr_eq(&five, &other_five));

// Ex. 4: Tracking Counts
use std::sync::Arc;

let five = Arc::new(5);
let _weak_five = Arc::downgrade(&five);

// This assertion is deterministic because we haven't shared
// the `Arc` or `Weak` between threads.
assert_eq!(1, Arc::weak_count(&five));
assert_eq!(1, Arc::strong_count(&five));