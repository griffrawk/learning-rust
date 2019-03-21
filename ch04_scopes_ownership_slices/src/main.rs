// simple ownerships
/*
fn main() {
    let mut fred = String::from("one");
    println!("{}",fred);

    fred = "four".to_string();
    println!("{}",fred);

    fred.push_str(", world!");
    println!("{}",fred);

    let bobby = String::from("arse");

    let s1 = String::from("hello");
    let mut s2 = s1.clone();   // new copy on heap
    s2.push_str(", Janet");

    println!("{}, world!", bobby);
    println!("{}", s2);
}
*/

// ownerships and functions
/*
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    println!("{}", s);   // so this won't work

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

    println!("{}", x);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

*/

// Using references to retain ownership
/*
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);  // s1 is still valid as only a ref was passed
}

fn calculate_length(s: &String) -> usize {
        s.len()
}

*/

fn main () {
    let bob = "fred";
    let bob= "charlie";
    println!("{}",bob);
}