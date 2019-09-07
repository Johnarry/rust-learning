// Boolean
let x = true;
let b: bool = false;             // ⚓ No TRUE, FALSE, 1, 0


// Char: a single Unicode scalar value
let x = 'x';
let y = '😎';                   // ⚓ No "x", only single quotes, because of Unicode support, cha is not a single byte, but four.


// Tuple: fixed-size rdered list of elements of different(or same) data types. 
let a = (1, 1.5, true, 'a', "Hello World");
// a.0 = 1; a.1 = 1.5; ...

let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;    // destructuring

let (e, _, _, _, _) = a;



// Array: fixed-ze list of elements of same data type
// Array useful when you want your data on the stack rather than the heap

let a = [1, 2 ,3]   // using comma to separate elements
let a: [i32; 5]  = [1, 2, 3, 4, 5];     // the number 5 indicates there are 5 element in the array

let a = [3; 5]      // Using semicolon (;) equal to --> let a = [3, 3, 3, 3, 3];

// Accessimg array Element
let first = a[0];



// Slice: allows to create a view/reference to access only that part of data. Syntax [starting_index..ending_index]
// strings literal is slice strings. slice can use on either string literal or String. 
// 💡 In general, you should use String when you need ownership and &str when you just need to borrow a string.
let a = [1, 2, 3, 4];

let b = &a[1..3];




// ⭐ ⭐ ⭐ Match Expression
// All possible case have to be included in match arms. That why _ => 
let tshirt_width = 20;
let tshirt_size = match tshirt_width {
    16 => "S",   // check 16
    17 | 18 => "M",  // check 17 and 18
    19 ... 22 => "L",  // Check from 19 to 22 (19, 20, 21, 22)
    22 => "XL",
    _ => "Not Available",
};

println!("{}", tshirt_size); // L



let is_allowed = false;
let list_type = match is_allowed {
    true => "Full",
    false => "Restricted"   // Because data type of is_allowed is boolean and all possibilities checked on conditions. So no _ => "" required
}
println!("{}", list_type);



let marks_paper_a: u8 = 25;
let marks_paper_b: u8 = 30;
let output = match (marks_paper_a, marks_paper_b) {
    (50, 50) => "Full marks for both papers",
    (50, _) => "Full marks for paper A",
    (_, 50) => "Full marks for paper B",
    (x, y) if x > 25 && y > 25 => "Good",     // 💡 Using if conditional statement
    (_, _) => "Work Hard"
}
println!("{}", output);  // work hard



// While
let mut a = 1;
while a <= 10 {
    println!("Current value : {}", a)
    a += 1; // No ++ or -- in Rust
}

let mut b = 0;       // usage of continue and break statement
while b < 5 {
    if b == 0 {
        println!("Skip value : {}", b);
        b += 1;
        continue;
    } else if b == 2 {
        println!("Break At : {}", b);
        break;
    }

    println!("Current value : {}", b);
    b += 1;
}


let mut c = 1;
'outer_while: while c < 6 { // Set label outer_while
    let mut d = 1;
    'inner_while: while d < 6 {
        println!("Current Value : [{}][{}]", c, d);
        if c == 2 && d == 2 { break 'outer_while;  }   // 💡 kill outer_while
        d += 1;
    }
    c += 1;
}

loop {
    //Do something here
    //Using continue, break statement and logical operation
}

for a in 0..10 { // (a = 0; a < 10; a++) in other language
    println!("Current value : {}", a); // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
}

// Working with arrays/vectors
let group: [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];


for n in 0..group.len() {
    //Do something Bad Practice 👎
}

for person in group.iter() {
    // Do Something 👍
}