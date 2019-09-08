/**
 * Every reference in Rust has lifetime, which is the scope for which that reference is valid
 * Most of the time lifetimes are implicit and inferred, just like most of the time , types are inferred
 * We must annotate lifetimes when the lifetimes of references could be related in a few different ways
 * Rust requires ussto annotate the relationships using generic lifetime parameter to ensure the actual references used at runtime will definitely be valid
 * 
 * Lifetime annotations dont change how long any of the references live.   
 * Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
 * One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other
 * The lifetimes parameters must start with an apostrophe( ' )
 */

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime


//! Lifetime Annotations in Function Signatures

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} // Will not compile !!!!!!!!

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} // Generic lifetime parameter was used to express that all references in the parameters and the return value must have the same lifetime.

// In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in
// Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned
// Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints
// Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature
// When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help

// 💡 Note
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());  // the result will have the lifetime of the parameter that has smaller lifetime which is ~string2
        println!("The longest string is {}", result);
    }
} // This code works fine

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());  // the result will have the lifetime of the parameter that has smaller lifetime which ~string2
    }
    println!("The longest string is {}", result);
} //! This will get an Error



//! Thinking in Terms of Lifetimes

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
} // This works fine

/**
 * In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, 
 * because the lifetime of y does not have any relationship with the lifetime of x or the return value
 */


/**
 * When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter 
 * for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created 
 * within this function, which would be a dangling reference because the value will go out of scope at the end of the function
 */
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string"); // because result will go out of scope at the end of the function
    result.as_str()
} // Won't work !!!!!!!!


//! Lifetime Annotations in Struct Definitions

// So far, we've only defined structs to hold owned types. It's possible for structs to hold references, but in that case we would need to add a lifetime annotation

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

//  This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field


//! Lifetime Elision

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}   // This will compile 


// 💡 There are rules that Rust can implicitly infer lifetime parameter for programmers
//* Lifetimes on function or method parameters are called ~input lifetime, and lifetimeson return values are called output lifetimes
/**
 * There are 3 rule that compiler use to figure out what lifetimes references have when there aren't explicit annotations. 
 * The first rule applies to input lifetimes and the second and third rules apply to output lifetimes If the compiler gets to 
 * the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop 
 * with an error. These rules apply to fn definitions as well as impl blocks.
 */

/**
 * 1st rule: each parameter that is a reference gets its own lifetime parameter. In other words, 
 * a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function 
 * with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on
 */

/**
 * 2nd rule: if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32
 */

/**
 * 3rd rule: if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, 
 * the lifetime of self is assigned to all output lifetime parameters
 */


//! the Static Lifetime

/**
 * One special lifetime we need to discuss is 'static, which means that this reference can live for the 
 * entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:
 */
let s: &'static str = "I have a static lifetime.";


//! Generic Type Parameters, Trait Bounds, and Lifetimes Together

 use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}