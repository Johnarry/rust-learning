fn main() {
    println!("{}, {}!", "Hello", "world"); // Hello, World!
    println!("Call me {0} or {0} {1}", "John", "Arryn"); // Call me John or John Arryn
    println!("{greeting}, {name}!", greeting="Hello", name="world");

    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3])
    /*
        [
            1,
            2,
            3
        ]
    */

    
}