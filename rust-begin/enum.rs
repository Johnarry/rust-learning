// Enum and Pattern Matching
// Enums allow you to define a type by enumerate its possible values. 
// Enumerate is usful when we know all the possibility. 


/**
 * For example, any IP address can be either a version four or six address, but not at the same time. 
 * That property of IP address make enums data structure approriate, because enum value can only be one of the variants.
 * As they are still fundamentally IP address, they should be treated as the same type
 */

enum IpAddrKind {
    V4,
    V6,
} // IpAddrKind is now a custom data type that we can use elsewhere in our code.


// ! Creating instances of each of the two variants of IpAddrKind
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

/**
 * Note that the variants of the enum are namespaced under its identifier, 
 * and we use a double colon to separate the two. The reason this is useful is that 
 * now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. 
 * We can then, for instance, define a function that takes any IpAddrKind:
 */

fn route(ip_kind: IpAddrKind) { 
    //dosomething
}
// ! We can call this function with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);


// ! Using Enums as a type
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,  // the key kind has the value of type IpAddrKind
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};


// ! Declare data type stored inside each variants
enum IpAddr {
    V4(String),    // Both V4 and V6 have the type of String for their data
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

// ! Each Variants can have different type of data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

// The standard library IpAddr
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// 💡 You can put any kind of data inside an enum variant: String, Numerics type or Structs

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
            /* 💡 self is   Write(
                                "hello",
                                 )   
            */                     
    }
}

let m = Message::Write(String::from("hello"));
m.call();


// ! The Option Enum and Its Advantages Over Null Values
enum Option<T> {
    Some(T),
    None,
}

// ! Combination of Enums and Match Control Flow Operator
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// The diferen between ~if and ~match is the coin value can be any type rather than boolean-only
// The ~match arm has two parts: a pattern and some code. 
// The curly brackets can be used if you want to run multiple lines of code in a match arm. 

// ! Patterns that bind to Values
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


value_in_cents(Coin::Quarter(UsState::Alaska)) // The binding for ~state will be UsState::Alaska 

// ! Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// !Matches Are Exhaustive
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}  // This Function will not compile intead it will cause a bug
// Because in Option<T> ther is still ~None variant so we miss the case

//! The _ Placeholder
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}


