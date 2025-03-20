use std::{io, thread};
use std::time::{Instant, Duration};
/*
Overview


Structs
    1. Thread
        - Two ways to get: after calling thread on the JoinHandle, or through the thread::current function
    2. Scope
    3. ScopeJoin
    4. LocalKey
        - Owned by the local thread, but can be shared across threads (no mutable borrows)
    5. JoinHandle - returned by the call to spawn
        - The join method returns a thread::Result containing Ok of the final value produced by the spawned thread, or Err of the value given to a call to panic! if the thread panicked.
    6. Builder
        - Build a new thread before it's spawned; set the name and stack size
        - Dependent (right now 2 MiB), pass to Builder::stack_size

Functions
    1. current() -> Thread
        - Returns a handle to the thread
    2. park_timeout(dur: Duration)
        - Blocking call until event: timeout or the thread's token is made available
    3. spawn
        - If the join handle is dropped, the spawned thread will implicitly be detached
    4. available_parallelism() -> Result<NonZero<usize>>
        - Usually returns the number of CPUs
*/
spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static
    T: Send + 'static,

// Ex 1
let builder = thread::Builder::new()
                              .name("foo".into())
                              .stack_size(32 * 1024);

let handler = builder.spawn(|| {
    // thread code
}).unwrap();

handler.join().unwrap();

// Ex 2
fn main() -> io::Result<()> {
    let count = thread::available_parallelism()?.get();
    assert!(count >= 1_usize);
    Ok(())
}

// Ex 3
let timeout = Duration::from_secs(2);
let beginning_park = Instant::now();

let mut timeout_remaining = timeout;
loop {
    thread::park_timeout(timeout_remaining);
    let elapsed = beginning_park.elapsed();
    if elapsed >= timeout {
        break;
    }
    println!("restarting park_timeout after {elapsed:?}");
    timeout_remaining = timeout - elapsed;
}

// Ex 4
let handler = thread::Builder::new()
    .name("named thread".into())
    .spawn(|| {
        let handle = thread::current();
        assert_eq!(handle.name(), Some("named thread"));
    })
    .unwrap();

handler.join().unwrap();