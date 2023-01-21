//Ownership and Functions
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function... // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function, // but i32 is Copy, so it's okay to still use x afterward
 }  // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
                            
 fn takes_ownership(some_string: String) { // some_string comes into scope
println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
 // memory is freed.
                        
fn makes_copy(some_integer: i32) { // some_integer comes into scope
 println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



// The concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.
// But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. 
//
// If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
// }


// fn main() {
//     // The double colon (::) is an operator that allows us to namespace this particular from function under the String type
//     // rather than using some sort of name like string_from.
//     let mut s = String::from("hello");
//     // String can be mutated but literals cannot.
//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // this will print `hello, world!`
// }


// fn main() { // s is not valid here; not yet declared.
//     let s = "hello"; // s is valid
//     // do stuff with s
// } // this scope is now over, s is no longer valid.

// Ownership Rules
//
// - Each value in Rust has a variable that’s called its owner.
// - There can be only one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
