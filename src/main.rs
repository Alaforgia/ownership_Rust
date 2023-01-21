//Ownership and Functions
fn main() {
    
}




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
