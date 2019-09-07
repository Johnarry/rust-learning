// Structs are similar to tuples, the pieces of the struct can be different types. A ( key : value ) pair


// Defining and Instantiating Structs

struct User {
    username: String,               // username, emil, sign_in_account, active are called fields
    email: String,
    sign_in_acount: u64,
    active: bool
}

//  We can change a value by using the dot notationand assigning into a particular field. 
// Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");


// Return a new instance from a function

fn build_user (email: String, username: String) -> User {
    User { 
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}


// Using the field init shorthand when Variables and Fields have the same name
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// Creating Instances from other Instances with Struct Update Syntax
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};


// Using Tuple Structs without 👉Named Fields to Create Different Types
// It looks like a Tuple, define by using struct statment but don't have struct fields

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// 💡 Note that the 👉black and 👉origin values are different types, because they're instances of different structs.
// 💡 A function that takes a parameter of type 👉Color cannot take a 👉Point as an argument even though they are oth made up of 3 i32 values.  
// 💡 Tuple struct behave like tuples: you can destruction them, you can use a . followed by the index to access an individual value


// Unit-Like Structs Without Any Fields
// You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. 
// Unit-like structs can be useful in situations in which you need to implement a trait on some type 
// but don’t have any data that you want to store in the type itself.




// Example for Implementation
//Using Struct to Calculating Area of a Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


// Adding Useful Functionality with Derived Traits

#[derive(Debug)]     // This annotation Enable print out debugging information
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);  // You can use either {:?} or {:#?}. These specifierstells println! we want to use an output format call Debug
}





// METHOD
// Their parameter is always 👉self, which represents the instance of the struct the method is being called on

// Defining Method

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {      // &self -> &Rectangle 
        self.width * self.height
    }                            // Note that we still need to use the & before self. Methods can take ownership of self
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
/**
 * We’ve chosen &self here because we don’t want to take ownership, and we just want to read the data in the struct, not write to it. 
 * If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use 👉&mut self as the first parameter. 
 * Having a method that takes ownership of the instance by using just self as the first parameter is rare; 
 * this technique is usually used when the method transforms self into something else and you want to prevent the caller from 
 * using the original instance after the transformation
 */


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



// Associated Function
// This type of function is defined within the imp block 
// but they don't tkae self as an parameter. They are call associated function because they are associated with struct
// They're still function, not methods, because they don't have an instance of the struct to work with
// Associated function are often used for constructors that will return a new instance of the struct
// To call this associated function, we used the :: syntax with the struct name


impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let sq = Rectangle::square(3);  // This function is namespaced by the struct: the :: syntax is used for both associated functions and
                                // namespaces created by modules


//Multiple imp Blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/**
 * There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax
 */