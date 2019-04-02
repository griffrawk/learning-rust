// The function signature now tells Rust that for some lifetime 'a, the function takes 
// two parameters, both of which are string slices that live at least as long as 
// lifetime 'a. The function signature also tells Rust that the string slice returned 
// from the function will live at least as long as lifetime 'a. 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    println!("Lifetimes!");
/*
    let x = 5;
    let r = &x;
    println!("r: {}, x: {}", r, x);
*/
    let string1 = String::from("gabcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    let result;     // defined but not init
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // result borrows string2, but string2 does not live past the end of the block above
    // if this is done in the block 
    // above then ok as result is then also discarded
    // println!("The longest string is {}", result);  

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = dbg!(ImportantExcerpt { part: first_sentence });
    println!("Nearly...");
}
