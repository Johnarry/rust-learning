//! Removing Duplication
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

//! Generic in Function Definition
// * fn largest<T>(list: &[T]) -> T { 


// Without Generic type
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// With Generic
n largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Todo: Not Done here yet
// the correct version
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/**
 * In the body of largest we wanted to compare two values of type T using the greater than (>) operator. 
 * Because that operator is defined as a default method on the standard library trait std::cmp::PartialOrd, 
 * we need to specify PartialOrd in the trait bounds for T so the largest function can work on slices of 
 * any type that we can compare. We don’t need to bring PartialOrd into scope because it’s in the prelude
 * 
 * With our non-generic versions of the largest function, we were only trying to find the largest i32 or char.
 * As types like i32 and char that have a known size can be stored on the stack, so they implement the Copy trait. 
 * But when we made the largest function generic, it became possible for the list parameter to have types in it 
 * that don’t implement the Copy trait. Consequently, we wouldn’t be able to move the value out of list[0] and 
 * into the largest variable, resulting in this error
 * 
 * To call this code with only those types that implement the ~Copy trait, we can add Copy to the trait bounds of T. 
 * A generic ~largest function that will compile as long as the types of the values in the slice that we pass into 
 * the function implement the ~PartialOrd and ~Copy traits, like ~i32 and ~char do
 * 
 * If we don’t want to restrict the largest function to the types that implement the Copy trait, 
 * we could specify that T has the trait bound Clone instead of Copy. Then we could clone each value 
 * in the slice when we want the largest function to have ownership. Using the clone function means 
 * we’re potentially making more heap allocations in the case of types that own heap data like String, 
 * and heap allocations can be slow if we’re working with large amounts of data
 * 
 * Another way we could implement largest is for the function to return a reference to a T value in the slice. 
 * If we change the return type to &T instead of T, thereby changing the body of the function to return a reference, 
 * we wouldn’t need the Clone or Copy trait bounds and we could avoid heap allocations. Try implementing these 
 * alternate solutions on your own.
 */

//! In Struct Definition
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}  // Because We only use one ~T generic type so both x and y hae t be the same type regardless of what type it is

//  <-- x and y doesnt have to be the same type -->
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}


//! In Enums Definition
enum Option<T> {
    Some(T),
    None,
}


enum Result<T, E> {
    Ok(T),
    Err(E),
}

/**
 * The ~Result enum is generic over two types, ~T and ~E, and has two variants: ~Ok, which holds a value of type ~T, and ~Err, 
 * which holds a value of type E. This definition makes it convenient to use the Result enum anywhere we have an operation 
 * that might succeed (return a value of some type T) or fail (return an error of some type E). Were T was filled in with the type std::fs::File 
 * when the file was opened successfully and E was filled in with the type std::io::Error when there were problems opening the file
 */


//! In Method Definitions

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {     // *  <-----
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// By declaring ~T as a generic type after ~impl, Rust can identify that the type in the angle brackets in ~Point is a generic type rather than a concrete type.
// Because we could implement methods only on ~Point<f32> instances

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
} // only apply for the ~Point instance with type ~f32 as a parameter


// Generic can be write like this, be used to it. Don't be surprised
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


