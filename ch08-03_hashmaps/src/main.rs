#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    // inserting K/V into map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}",scores );

    // 
    let teams  = vec![String::from("Bloo"), String::from("Yella")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter()    // iterates over teams, <_,_> doesn't assume types
        .zip(initial_scores.iter())         // and zips it with an iter over initial_scores
        .collect();                         // turns the iter into the hashmap as specced by let
    println!("{:?}", scores);
}