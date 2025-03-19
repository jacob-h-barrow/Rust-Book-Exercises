// Read this first
/*
- Advanced Traits
    - Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures
        - We dont need to annotate types because we cant implement a trait on a type multiple time... can only choose the type of Item once
        
    - Two ways to use default type parameters
        1. To extend a type without breaking existing code
        2. To allow customization in specific cases most users won't need
        
    - Newtype Pattern 
        - Lightweight way to encapsulate implementation details
        - IMPORTANT: No runtime performance penalty
        - IMPORTANT: the wrapper type is elided at compile time
        - IMPORTANT: methods are not inherited for Newtypes (default); instead, implementation of the Deref trait would be required for structural subtyping/Newtype methods inheritance
            - https://doc.rust-lang.org/book/ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
            
- Advanced Closure Returns
    - Since each closure is also its own distinct type, you will need to do one of two things
        1. Concrete specific: use impl
        2. Same signature, but different implementations: use a trait object
*/
/*
    Example One: Iterator
    
    - Why not define the Iterator with pure generics?
        - When a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time
        - With associated types, we don't need to annotate types because we can't implement a trait on a type multiple times
            - Associated types also becom part of the trait's contract
*/
pub trait Iterator {
    // The placeholder
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Example Two: Generic Type Parameters and Operator Overloading
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Specifying a default type when declaring a generic type; called default type parameters  
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// Example 3: Newtype pattern precursor
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Example 4: Newtype Customization (Generic Binding)
use std::fmt;

struct Wrapper(Vec<String>);

// The Display trait used the Vec<T> type defined outside of the crate
//// Here Wrapper<T> will work, but really be Wrapper<String>
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

// Example 5: Type Alias
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
// Example 5: Type Alias
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
// Example 6: Overriding a type's default/known size at compile time
//// A trait bound on ?Sized means “T may or may not be Sized”
//// Only used with teh Sized trait
//// Notice that t: &T is a reference because we have to use some kind of pointer to accomplish this
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
// Example 7: Advanced Closure Returns
/*
- Advanced Closure Returns
    - Since each closure is also its own distinct type, you will need to do one of two things
        1. Concrete specific: use impl
        2. Same signature, but different implementations: use a trait object
*/
//// Sub-1: Concrete Return Type
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

//// Sub-2: Trait Object Return Type
fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
