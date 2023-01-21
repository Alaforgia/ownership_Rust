// The Slice Type
// Slice type does not have ownership
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
fn main() {
    let mut s = String::from("hello world");
    let _word = first_word(&s); // word will get the value 5

    // A string slice is a reference to part of a String
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // if you want to start at the first index (zero), you can drop the value before the two periods. In other words, these are
    // equal:
    // let slice = &s[0..2];
    // let slice = &s[..2];
    //
//  You can also drop both values to take a slice of the entire string. So these are equal:
// let s = String::from("hello");
// let len = s.len();
// let slice = &s[0..len];
// let slice = &s[..];

    s.clear() // this empties the String, making it equal to ""

    // println!("the first word is: {}", word);

} // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with. 
// word is now totally invalid!


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // as_bytes method converts our String to an array of bytes

    for (i, &item) in bytes.iter().enumerate()  { //iter is a method that returns each element in a collection and that 
// enumerate wraps the result of iter and returns each element as part of a tuple instead.
// The first element of the tuple returned from enumerate() is the index, the second is a reference to the element.
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}





//Rules of References
// At any given time, you can have either but not both of the following: one mutable reference or any number of immutable references.
// References must always be valid.


// Dangling References
// fn main() {
//     let _reference_to_nothing  = dangle();
// }

// fn dangle() -> String {  // dangle returns a reference to a String
//     let s =String::from("hello"); // s is a new String

//     s // we return a reference to the String, s
// }  // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!






// Mutable References
// fn main() {
//     // You can have only one mutable reference to a particular piece of data in a particular scope. 
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }





// // References and Borrowing
// fn main() {
// // This is how you would define and use a calculate_length function that has a reference to an object as a parameter 
// // instead of taking ownership of the value.
//     let s1 = String::from("hello");
// // The ampersands are references, they allow you to refer to some value without taking ownership of it.
//     let len = calculate_length(&s1);

//     println!("the length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.




// Return Values and Scope
// fn main() {
//     let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
//     let s2 = String::from("hello"); // s2 comes into scope
//     let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, 
//     //which also moves its return value into s3
//    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. 
//    // s1 goes out of scope and is dropped.

//     fn gives_ownership() -> String { // gives_ownership will move it return value into the function that calls it
//         let some_string = String::from("hello"); // some_string comes into scope
//         some_string // some_string is returned and moves out to the calling function
//        }

//       // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//     // scope
//     a_string // a_string is returned and moves out to the calling function
//    }





//Ownership and Functions
// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function... // ... and so is no longer valid here

//     let x = 5; // x comes into scope

//     makes_copy(x); // x would move into the function, // but i32 is Copy, so it's okay to still use x afterward
//  }  // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
                            
//  fn takes_ownership(some_string: String) { // some_string comes into scope
// println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//  // memory is freed.
                        
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//  println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.



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
