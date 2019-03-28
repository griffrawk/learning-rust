fn main() {
    /*
    // Listing 10-1: Code to find the largest number in a list of numbers
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // Listing 10-2: Code to find the largest number in two lists of numbers
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // Listing 10-3: Abstracted code to find the largest number in two lists
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largestf(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largestf(&number_list);
    println!("The largest number is {}", result);
    */

    /*
    // Listing 10-4: Two functions that differ only in their names and the types in their signatures
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    */

    // Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t compile yet
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['t', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different types
    #[derive(Debug)]
    struct Point<T, U> {
        // the generic can be T *or* U
        x: T,
        y: U,
    }
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = dbg!(Point { x: 5, y: 4.5 });
    let integer_and_string = dbg!(Point {
        x: 5,
        y: "fred".to_string()
    });
}

// fn for 10-3
fn largestf(list: &[i32]) -> i32 {
    // passed a ref to slice of i32, returns an i32
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn for 10-4
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn for 10-5
fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy, {
    // function largest is generic over some type T. This function has one parameter named list, which is // a slice of values of type T. The largest function will return a value of the same type T. Using
    // PartialOrd allows types T that impl it to be compared, and Copy trait allows types T that are
    // copyable
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
