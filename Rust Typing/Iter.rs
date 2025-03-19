/*
Iterator
- Three forms of iteration
    1. iter(), which iterates over &T.
    2. iter_mut(), which iterates over &mut T.
    3. into_iter(), which iterates over T. This consumes the collection/can't be used again.

- All Iterator types implement the IntoIterator traits by just returning themselves 

- Structs
    - Chain
    - Enumerate
    - Filter, FilterMap
    - Zip
    - SkipWhile
    - Take, TakeWhile
    - Map, MapWhile

- Adapters/Functions: iterator adapters are lazy, must call next on them to do work
    - from_fn
    - zip
    - chain
    
- Traits  (Just Need To Know Exist)
    - Iterator
    - Sum
    - Extend
*/
// IntoIterator Ex 1
let mut values = vec![41];
for x in values.iter_mut() {
    *x += 1;
}
for x in values.iter() {
    assert_eq!(*x, 42);
}
assert_eq!(values.len(), 1); // `values` is still owned by this function.

// IntoIterator Ex 2
let mut values = vec![41];
for x in &mut values { // same as `values.iter_mut()`
    *x += 1;
}
for x in &values { // same as `values.iter()`
    assert_eq!(*x, 42);
}
assert_eq!(values.len(), 1);

/*
    Adapter Examples
*/

// Ex 1
let v = vec![1, 2, 3, 4, 5];

v.iter().for_each(|x| println!("{x}"));
//// or
for x in &v {
    println!("{x}");
}

// Ex 2
use std::iter::zip;

let xs = [1, 2, 3];
let ys = [4, 5, 6];

let mut iter = zip(xs, ys);

assert_eq!(iter.next().unwrap(), (1, 4));
assert_eq!(iter.next().unwrap(), (2, 5));
assert_eq!(iter.next().unwrap(), (3, 6));
assert!(iter.next().is_none());

// Nested zips are also possible:
let zs = [7, 8, 9];

let mut iter = zip(zip(xs, ys), zs);

assert_eq!(iter.next().unwrap(), ((1, 4), 7));
assert_eq!(iter.next().unwrap(), ((2, 5), 8));
assert_eq!(iter.next().unwrap(), ((3, 6), 9));
assert!(iter.next().is_none());

// Exs
let a = [-1i32, 0, 1];

let mut iter = a.iter().take_while(|x| x.is_negative());

let mut iter = a.iter().filter(|x| x.is_positive());

let mut iter = a.iter().take(2);

let result: Vec<i32> =  a.iter()
                           .take_while(|n| **n != 3)
                           .cloned()
                           .collect();

assert!(a.iter().any(|&x| x > 0));
assert!(a.iter().all(|&x| x > 0));

let mut iter = (0..).take(3);

// At the heart of the Iterator trait
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Creating an iterator of your own
// First, the struct:

/// An iterator which counts from one to five
struct Counter {
    count: usize,
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `next()`'s implementation below.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Then, we implement `Iterator` for our `Counter`:

impl Iterator for Counter {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// And now we can use it!

let mut counter = Counter::new();

assert_eq!(counter.next(), Some(1));
assert_eq!(counter.next(), Some(2));
assert_eq!(counter.next(), Some(3));
assert_eq!(counter.next(), Some(4));
assert_eq!(counter.next(), Some(5));
assert_eq!(counter.next(), None);
