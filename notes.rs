/*
    - C++ Users
        - Data (w/o the Copy trait) which are shallow copied demonstrate weak pointers
        - Data (w/o the Copy trait) moved to a fn use move semantics (&&)
        - Data (w/o the Copy trait) passed as a mutable reference act like a shared pointer... deleted on end of fn

    - Python Users
        - Syntax is similar
            - The match statement
            - Function signatures
            - Staticmethod Ctors (impl fn without self -> Self)
            - Tuple packing and unpacking
            - Enumeration/iteration
            - Dataclasses written using Structs 

    - Scalar Types: integers, floating-pt, booleans, and chars

    - Numeric overflows caught in debug compiler mode, not release mode

    - Most control flow expressions dont require (), but require {}

    - Let If statements must have the same branch value types

    - The String type actually has an internal pointer... shallow copy doesnt copy this pointer
        - Shallow copy (assignments) might cause a lifetime compilation error
        - Use references (&String) or string views/literals (&str)
        - A &str covers both references to strings and literals
    
    - Slices let you reference a contiguous sequence of elements in a collection

    - Primitive data types (ones that have the Copy Trait implemented) are deep copied by default
        - Means you don't have to worry about lifetime errors
        - Data types can't have both the Copy and Drop Traits implemented (IMPORTANT)
    
    - Ownership
        - Can only have one mutable reference (without any const references)
        - Can have any amount of const references (without any mutable reference) 

    - Structs
        - If the instance is mutable, all fields must be mutable
        - (IMPORTANT) Using another instance to build a template... unless referenced, the data is moved to the new struct
            - This means the previous struct instance field that was moved can't use it!
*/


// Binding and Destructuring Tuples
/*
    // Automatic below
    let tup: (i32, f64, u8) = (500, 6.4, 1);
*/
let tup = (500, 6.4, 1);
let (x, y, z) = tup;

// Static Arrays - Three Ways
//// 1. Auto
let a = [1, 2, 3, 4, 5];
//// 2. Typed
let a: [i32; 5] = [1, 2, 3, 4, 5];
//// 3. Same value initialized
let a = [3; 5]; // [3, 3, 3, 3, 3]


// Let If
let condition = true;
let number = if condition { 5 } else { 0 };


// Range Creation (WORK ON)
for number in (1..4).rev() { HERE; }


// Copy
//// Deep Copy
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");

//// Stack-Only Data: Deep Copied by Default
let x = 5;
let y = x;

println!("x = {x}, y = {y}");

// Shadowing allowed per scope
let x = 5;

let x = x + 1;

{
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
}

println!("The value of x is: {x}");


/*
    Ownership
*/
fn example_main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let (s3, s2_len) = takes_and_gives_back(s2);

    change(&mut s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> (String, usize) {
    (a_string, a_string.len())
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//// Bad
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!



// Slice
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
let slice = &s[..];

assert_eq!(slice, &s);

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}



/*
    Three Ways To Instantiate A Struct
        1. Building from scratch: all are new
        2. Building from other by name: unless referenced, the data is moved to the new struct. 
            - This means the previous struct instance field that was moved can't use it!
        3. Building from other by ellipsis: same as by name. unless referenced, the data is moved to the new struct. 
            - This means the previous struct instance field that was moved can't use it!
            - The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
*/
//// Definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//// Build from scratch
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//// Build from other by name
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

//// Build from other by ellipsis
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}



/* 
    Method
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // Python Users: similar to class generation using a staticmethod
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the rectangle is {} square pixels",
        Rectangle::square(3).area()
    );
}
