use std::sync::atomic::Ordering::{Relaxed, Release, Acquire, AcRel, SeqCst}
/*
- A happens-before relationship is formed when an acquire-load operation observes the result of a release-store operation.

- The truth is that the memory model doesn’t say anything about timing at all. It only defines in which order certain things happen; not how long you might have to wait for them. 

Orderings in Rust:
    - Relaxed ordering: {Relaxed}
        - Everything that happens within the same thread happens in order
        - Better said: relaxed memory provides a guaranteed total modification order of each individual atomic variable
            - Means that all modifications of the same atomic variable happen in an order that is the same from the perspective of every single thread
            - Even if there’s more than one possible order of modification for an atomic variable, all threads will agree on a single order.
        - On all modern platforms, relaxed load and store operations compile down to the same processor instructions as non-atomic reads and writes.

    - Release and acquire ordering: {Release, Acquire, AcqRel}
        - Used to form a happens-before relationship between threads
        - Release applies to store ops
        - Acquire applies to load ops
        - To get both use AcqRel, important for consistent ops: fetch-and-modify or compare-and-exchange

    - Sequentially consistent ordering: {SeqCst}
*/

// Relaxed
/*
    Possible outcomes:
        - (0, 0), (10, 20), (10, 0), (0, 20), ...
    
    Why?
        - Ordering within a and b is guaranteed, but there's no happens-before guarantee between a and b
        - Thread executing b doesnt guarantee the execution point of a
*/
static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed); 1
    Y.store(20, Relaxed); 2
}

fn b() {
    let y = Y.load(Relaxed); 3
    let x = X.load(Relaxed); 4
    println!("{x} {y}");
}

// Release and Acquire
//// Ex 1
static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // Everything from before this store ..
    });
    while !READY.load(Acquire) { // .. is visible after this loads `true`.
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));
}

//// Ex 2
static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}

// Extra
use std::sync::atomic::AtomicPtr;

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(), p, Release, Acquire
        ) {
            // Safety: p comes from Box::into_raw right above,
            // and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}