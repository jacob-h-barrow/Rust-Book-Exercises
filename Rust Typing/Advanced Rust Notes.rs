// Read this second
/*
    - Bounded Parametric Polymorphism
        - Rust uses generics to abstract over different possible types and trait bounds
        - A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime
            - Create a trait object by specifying a pointer, such as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait
            - Trait objects are similar to protocols in Python
                - Duck typing or structural subtyping
                - HUGE: Done at compile time for Rust
            - Differs from generic type parameter (requires one type substitution), whereas trait objects can allow for multiple concrete types to fill in for the trait object at runtime
        - IMPORTANT: generics supports homogeneous collections, trait objects supports non-homogeneous ones!!!
        - IMPORTANT: generics permit monomorphization using static dispatch, trait objects using dyn types do dynamic dispatch at runtime!!!
        
    - Structing Deeper
        - IMPORTANT: even if the struct is made public, the fields are private unless noted
*/
// OOP Example

pub trait Draw {
    fn draw(&self);
}

// OPTION ONE: Non-homogeneous collection through a trait object
pub struct Screen {
    // This vector is of type Box<dyn Draw>, which is a trait object
    //// Its a stand-in for any type inside a Box that implements the Draw trait
    //// Important: allows for a non-homogeneous collection
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// OPTION TWO: Homogeneous collection through generics
pub struct Screen<T: Draw> {
    // This is important: this restricts us to a Screen instance of a homogeneous collection
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}


impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
///////////////////////////////////////////////////////////
// Example Two: OOP using the state pattern for a blog //
///////////////////////////////////////////////////////////
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    pub fn request_review(&mut self) {
        /*
            To consume the old state, the request_review method needs to take ownership of the state value. This is where the Option in the state field of Post comes in: we call the take method to take the Some value out of the state field and leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. This lets us move the state value out of Post rather than borrowing it. Then we’ll set the post’s state value to the result of this operation.
            
            We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review(); to get ownership of the state value. This ensures Post can’t use the old state value after we’ve transformed it into a new state.
        */
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    
    pub fn content(&self) -> &str {
        /*
            We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value. Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned
            
            We then call the unwrap method, which we know will never panic, because we know the methods on Post ensure that state will always contain a Some value when those methods are done. 
            
            At this point, when we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.
        */
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
