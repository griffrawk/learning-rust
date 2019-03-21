fn main() {
    println!("Hello, you big round blue world!");
    let x: u64 = 15;

    dbg!(factorial(x));
}

fn factorial(n: u64) -> u64 {
    if dbg!(n <= 1) {
        dbg!(n)
    } else {
        dbg!(n * factorial(n - 1))
    }
}