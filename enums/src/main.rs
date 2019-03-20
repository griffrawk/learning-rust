// these defs have to move so both main and bob can use them
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    println!("Value is {}", value_in_cents(Coin::Quarter(UsState::Arkansas)));
    println!("Value is {}", value_in_cents(Coin::Nickel));

    let test_enum = Coin::Nickel;
    let another_enum = dbg!(Coin::Quarter(UsState::Alaska));
    let yet_another = Coin::Quarter(UsState::Arkansas);
    let blah = Coin::Nickel;
    let foo = UsState::Alabama;
    

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = dbg!(Some(5));
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // these next two do the same thing, but if let doesn't need the default _
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(5) = some_u8_value {
        println!("five");
    }
    println!("{:?}",some_u8_value);

    bob(); //can bob use value_in_cents if its defined in the main()? No so I moved it.
}

fn bob() {
    println!("Value is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}