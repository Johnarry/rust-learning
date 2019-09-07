// Vector is a re-sizable data structure, Vector allows you to store more than one value in a single data structure that puts all the values
// next to each other in memory. Vector can only store value of the 👉 same type.
// Vector is a generic type, written as 👉Vec<T> in which 👉T can be any type.


//  Create vectors - 2 ways
let mut a = Vec::new();   
let mut b = vec![];

// Creating with data types
let mut a2: Vec<i32> = Vec::new();  // Notice we added notation here because there no values had been inserted so Rust doesnt know which type do we want
let mut v = vec![1, 2, 3];          // Rust can infer the type o value you want to store

// Updating a Vector
let mut v1 = Vec::new();   // make it mutable before changing any value of a vector
v1.push(5);
v1.push(6);
v1.push(8);


// 💡 Reading Elements of Vectors
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {  // the get method return an Option<&T>
    Some(third) => println!("The third elemnt is : {}", third);   
    None => println!("There is no third element.");
}


// Handle panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];     // Will cause the programm to panic. Useful when you want program to crash 
                                  // if there's an attempt to access an element pass the end of the vector
let does_not_exist = v.get(100)   // Return None without panic. Useful when accessing an element pass the end of 
                                  // the vector happen occasionally under normal circumstances


// Iterating over the Values in a vector
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // dereference operator (*) us to get the value in i before e can add 50 to it. Because the i itelf is a pointer to the actual value store in the vector 
}

// Using an Enum to Store Multiple Types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12)
];

/** Rust needs to know what types will be in the vector at compile time so it know exactly how much memory on the heap will be needed to store each element.
 A secondary advantage is that we can be explicitly about what types are allowed in this vector. Because with multiple data type stored in vector problem can happen 
 when you apply method on an element whose data type doesnt support
*/