// Function in Rust
// 👉 When using arguments, you must declare data types.
// 👉 Rust is an @expression-based language. Distinguishing statement and expression.
// 👉 Statement are instructions that perform some action and @do not return value (let is statement, fn is statement).
// 👉 Expressions evaluate to a resulting value (5 + 6, number 6 in let a = 6, calling a function, calling a macro, create new scope {}).
// 👉 By default functions return empty tuple (). If you want to return a value, return type must be specified after ( -> )
// 👉 In Rust the return value of the function is the final expression in the body of a function without a semicolon(;)




fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}


fn plus_one(a: i32) -> i32 {
    a + 1           // No semi colon, this is equal to return a + 2; We should only use return on conditional returns (an early return)
}

// 🌟Function pointers, Usage as a Data Type
//let b = plus_one;
//let c = b(5);

//let b: fn(i32) -> i32 = plus_one; // same, with type inference
//let c = b(5); // 6



//let b = plus_one;
fn plus_two(b: fn(i32) -> i32, x: i32) -> i32 {
    b(b(x))
}
