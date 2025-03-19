/*
Overview
    - Simplest form of heap allocation
    - Ownership for allocation, and drops contents when out of scope
    - Memory layout allocation isnt needed to be known for the majority of cases

Methods
    - Experimental
        - allocator
        - as_mut_ptr/as_ptr 
        - try_new_zeroed 
    - leak
        - Consumes and leaks a mutable reference
        - Type T must outlive the chosen lifetime 'a
        - Usually used with data that lives the rest of the programs lifetime
    - new
        - If data is sized, then it allocates memory and places x into it
    
Traits (Just Need To Know Exist)
    - IntoIterator
    - AsMut/AsRef
    - Borrow/BorrowMut
    - Deref/DerefMut
    - Display
    - Drop
    - Eq
    - Fn/FnMut/FnOnce
    - Ord/PartialEq/PartialOrd
    - TryFrom<Box<[T]>>
*/
let boxed_slice: Box<[i32]> = vec![0; 3].into_boxed_slice();

// IntoIterator
//// You can explicitly iterate a boxed slice by value using `IntoIterator::into_iter`
for item in IntoIterator::into_iter(boxed_slice).enumerate() {
    let (i, x): (usize, i32) = item;
    println!("boxed_slice[{i}] = {x}");
}

/*
pub fn leak<'a>(b: Box<T, A>) -> &'a mut T
where
    A: 'a, 
*/
// Example 1: Unsized data
let static_ref = Box::leak(boxed_slice);
static_ref[0] = 4;
assert_eq!(*static_ref, [4, 2, 3]);

// Example 2: Simple usage
let x = Box::new(41);
let static_ref: &'static mut usize = Box::leak(x);
*static_ref += 1;
assert_eq!(*static_ref, 42);

/*
pub fn new(x: T) -> Box<T>
*/
let five = Box::new(5);
