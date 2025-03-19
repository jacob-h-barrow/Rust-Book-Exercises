use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/*
- Experimental: ReentrantLock {Self, Guard}

Mutex
    - Data can only be accessed through the RAII guards returned from lock and try_lock... returns a Result

    - Poisoned if a thread which holds a lock panics; no one can access the data
        - Can beat the system: the PoisonError type has an into_inner method to return the "successful" guard/lock... can access data this way!

    - Methods
        1. into_inner(self) -> LockResult<T> where T: Sized
            - Consumes this mutex and returns the data
        2. get_mut(&mut self) -> LockResult<&mut T>
            - The mutable borrow statically guarantees no locks exist
        3. try_lock(&self) -> TryLockResult<MutexGuard<'_, T>>
            - Gets lock or Err returned
                - Poisoned error
                - WouldBlock error if lock already acquired
        4. clear_poison(&self)

RwLock
    - Multiple readers, one writer; no guarantees on the priority policy of the lock

    - Methods
        1. try_read(&self) -> TryLockResult<RwLockReadGuard<'_, T>>
            - similar to the Mutex.try_lock()
        2. try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, T>>
            - similar to the Mutex.try_lock()
        3. clear_poison(&self)
        4. into_inner(self) -> LockResult<T> where T: Sized
        5. get_mut(&mut self) -> LockResult<&mut T>
*/
// Mutex Examples
//// Ex 1
let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

thread::spawn(move || {
    let mut lock = c_mutex.try_lock();
    if let Ok(ref mut mutex) = lock {
        **mutex = 10;
    } else {
        println!("try_lock failed");
    }
}).join().expect("thread::spawn failed");
assert_eq!(*mutex.lock().unwrap(), 10);

// Ex 2 - Clear the Poisoned state
let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

let _ = thread::spawn(move || {
    let _lock = c_mutex.lock().unwrap();
    panic!(); // the mutex gets poisoned
}).join();

assert_eq!(mutex.is_poisoned(), true);
let x = mutex.lock().unwrap_or_else(|mut e| {
    **e.get_mut() = 1;
    mutex.clear_poison();
    e.into_inner()
});
assert_eq!(mutex.is_poisoned(), false);
assert_eq!(*x, 1);

// Ex 3 
let mutex = Mutex::new(0);
assert_eq!(mutex.into_inner().unwrap(), 0);

// Ex 4
let mut mutex = Mutex::new(0);
*mutex.get_mut().unwrap() = 10;
assert_eq!(*mutex.lock().unwrap(), 10);

// Ex 5
let lock = Arc::new(Mutex::new(0_u32));
let lock2 = Arc::clone(&lock);

let _ = thread::spawn(move || -> () {
    // This thread will acquire the mutex first, unwrapping the result of
    // `lock` because the lock has not been poisoned.
    let _guard = lock2.lock().unwrap();

    // This panic while holding the lock (`_guard` is in scope) will poison
    // the mutex.
    panic!();
}).join();

// The lock is poisoned by this point, but the returned result can be
// pattern matched on to return the underlying guard on both branches.
let mut guard = match lock.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(),
};

*guard += 1;

// RwLock Examples
//// Ex 1
let lock = RwLock::new(1);

let n = lock.read().unwrap();
assert_eq!(*n, 1);

assert!(lock.try_write().is_err());

//// Ex 2
let lock = RwLock::new(1);

match lock.try_read() {
    Ok(n) => assert_eq!(*n, 1),
    Err(_) => unreachable!(),
};

//// Ex 3
let lock = RwLock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write().unwrap();
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here