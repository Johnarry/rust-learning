// We use traits to define shared behavior in an abstract way. 
// We can use traits bounds to specify that a generic can be any type that has certain behavior.


//! Defining a Trait

/**
 * A type's behavior consists of the methods we can call on that type. Different types share the same behavior
 * if we can call the same methods on all of those type
 */

pub trait Summary {
    fn summarize(&self) -> String;
}

// We declare the method signatures that descrie the behaviors of the types that implement this trait
// After the method signature ~fn summarize(&self) -> String; instead of providing an implementation within curly brackets, we can use a semicolon. 
// Each type implementing this trait must provide itts own custom behavior for the body of the method. The compiler will enforce that any type that 
// has the ~Summary trait will have the method ~summerize defined with this signature exactly

// A trait have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon (:)

//! Implementing a Trait in a Type
//* impl Trait_Name for Type_Name
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location) // only within NewsArticle context
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) // only within Tweet context
    }
}

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize()); // It just likes a method call

/**
 * One restriction to note with trait implementations is that we can implement a trait on a type only if 
 * either the trait or the type is local to our crate. This rule ensures that other people's code can't 
 * break your code and vice versa
 */

//! Default Implementation
// We can define a default behavior for some or all of the methods in a trait instead of requiring implementations
// for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each 
// method's default behavior.
// To use a default implementation instead of defining a custom implementatio, we specify an empty ~impl block with
// impl Summary for NewsArticle {}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// <-- Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // We can call ~summerize_author method  here even though that method hasn't been 
                                                                   // implemented yet
    }
}

// To use this version of Summary, we only need to define ~summerize_author when we implement the trait on a type
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


//! Trait as Parameter
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize()); // ~item can use method summarize
}

// Instead of concrete type for the ~item parameter, we specify the ~impl keyword and the trait name. This parameter accept any methods on ~item
// that implements the specified trait. In the body of notify, we can call notify and pass in any instance if NewsArticle or Tweet. 
// Code that calls the function with any other type, such as a String or an i32, won't compile because those types don't implement ~Summary


//! Trait Bound Syntax
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}   // This is the concise form

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}    // The same as the above code

// Both way have been use to express the same implementation but sometime you need to use generic-like way, for example

pub fn notify(item1: impl Summary, item2: impl Summary) { // This function allow item1 and item2 to have different types

pub fn notify<T: Summary>(item1: T, item2: T) {  // This way we want to enforce both parameters to have the same type, 
                                                 // that's only possible to express using a trait bound


//! Specifying Multiple Trait Bounds with The + Syntax

// We can also specify more than one trait bound

pub fn notify(item: impl Summary + Display) {

// Alternative way
pub fn notify<T: Summary + Display>(item: T) {

//! Clearer Trait Bounds with ~where Clauses
// A Function with mutiple generic type parameters can contain lots of trait bound information between the function's name and its parameter list
// making the function signature hard to read

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

//* Alternative way
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{


//! Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {  // <--
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/**
 * By using ~impl Summary for the return type, we specify that the ~return_summarizable function returns some type that
 * implements the Summary trait without naming the concrete type. In this case, ~return_summarizable returns a Tweet, but
 * the code calling this function doesn't know that.
 * 
 * The ability to return a type that is only specified by the trait it implements is especially useful in the context
 * of closures and iterators. Closure and Iterator creates types that only the compiler knows or types that are very long to specify. 
 */


// However, you can only use ~impl Trait if you're return a single type

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}  // todo: Not gonna work, find out solution later in Chapter 17


//! Using Trait Bounds to Conditionally Implement Methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// the type Pair<T> always implements the new function. But Pair<T> only implements the cmp_display method if its inner 
// type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.

/**
 * We can also conditionally implement a trait for any type that implements another trait. 
 * Implementations of a trait on any type that satisfies the trait bounds are called ~blanket implementations 
 * and are extensively used in the Rust standard library. For example, the standard library implements the 
 * ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code
 */

impl<T: Display> ToString for T {
    // --snip--
}

/**
 * Because the standard library has this blanket implementation, we can call the to_string method defined by the 
 * ToString trait on any type that implements the Display trait. For example, we can turn integers into their 
 * corresponding String values like this because integers implement Display
 */

let s = 3.to_string();


/** Summary
 * Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also 
 * specify to the compiler that we want the generic type to have particular behavior. The compiler can then 
 * use the trait bound information to check that all the concrete types used with our code provide the correct 
 * behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type 
 * that the type didn’t implement. But Rust moves these errors to compile time so we’re forced to fix the 
 * problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior 
 * at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics
 */