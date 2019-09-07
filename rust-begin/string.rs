// Rust has only one string type in the core language, which is the string slice ~str that is usually seen in its borrowed form ~&str
// In Rust, when refers to ~string, they usually mean the ~String and the string slice ~&str types
// Both String and string slices are UTF-8 encoded

// ! Creating a New String
let mut s = String::new();

let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();      // The ~to_string method, which is available on any type that implements the ~Display trait

let s = String::from("initial contents");

// Because string are UTF-8 encoded, so we can include any properly encoded data in them, almost any character of any language in the world

//! Update a String

let mut s = String::from("foo");
s.push_str("bar");  // The ~push_str method takes a string slice because we don't necessarily want to take ownership of the parameter 

// This is goanna work as we expect
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);  // ~s2 can still be use because it isn't taken ownership

let mut s = String::from("lo");
s.push('l');   // The push method takes a single character as a parameter and adds it to the String

//! Concatenation with the + Operator or the format! Macro
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used but s2 is still be a valid String after this operation

// The + operator uses the add method, whose signature looks something like this
fn add(self, s: &str) -> String {}


let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
// Using format! is much easier to read and doesn’t take ownership of any of its parameters

//! Indexing into Strings
// Rust will not let you to indexing String
let s1 = String::from("hello");
let h = s1[0]; // will get an error

// Beause not all characters can be encoded within 1 byte


//!Slicing Strings
let hello = "Здравствуйте";

let s = &hello[0..4];  // Зд


// But
let hello = "Здравствуйте";

let s = &hello[0..1]; // Rust would panic at runtime in the same way as if an invalid index were accessed in a vector

//! Methods for Iterating Over Strings
for c in "नमस्ते".chars() {
    println!("{}", c);
}

for b in "नमस्ते".bytes() {
    println!("{}", b);
}