// Storing Keys with Associated Values in Hash Maps
// --> HashMap<K, V> stores a mapping of keys of type K to values of type V
// --> It does it via a hashing function, which determines how it places these keys and values into memory
// --> It likes Object in JavaScript and Dictionary in Python
// --> Like Vector, Hash Maps are homogeneneous: all of the keys must have the same type, and all of the values must have the same type


//! Creating a New Hash Map
use std::collections::HashMap; // use have to include it in the scope in order to use it

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Another way
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

/**
 * Another way of constructing a hash map is by using the ~collect method on a vector of tuples, 
 * where each tuple consists of a key and its value. The collect method gathers data into a number of collection types, including HashMap. 
 * For example, if we had the team names and initial scores in two separate vectors, we could use the ~zip method to create a 
 * vector of tuples where “Blue” is paired with 10, and so forth. Then we could use the ~collect method to turn that vector of tuples into a hash map.
 * The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures
 */

//! Hash Maps and Ownership
/**
 * For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, 
 * the values will be moved and the hash map will be the owner of those values
 */

use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!

//! Accessing Values in a Hash Map
// We can get a value out of the hash map by providing its key to the get method
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);   // Some(&10)

// We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

//! Updating a Hash Map
// Overwriting a value
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores); // {"Blue": 25}

// Only Inserting a Value If the Key Has No Value
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);   //{"Yellow": 50, "Blue": 10}

// Updating a Value Based on the Old Value
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {    // The or_insert method actually returns a mutable reference (&mut V) to the value for this key
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
