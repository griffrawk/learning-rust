fn main() {
    println!("Play with vectors!");
/*
    let v1: Vec<i32> = Vec::new();      // explicity
    println!("v1: {:?}", v1);

    let v2 = vec![1, 2, 3];     // inferred
    println!("v2: {:?}", v2);

    let mut v3 = Vec::new();    // not known until something pushed on

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("v3: {:?}", v3);

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("v3: {:?}", v3);

    let mut v4: Vec<i32> = vec![];     // empty allowed using macro? not unless type specified like v1
    println!("v4: {:?}", v4);

    v4.push(5);
    v4.push(6);
    v4.push(7);
    v4.push(8);
    v4.push(919);
    println!("v4: {:?}", v4);

    let v5 = vec![1, 2, 3];
    println!("v5: {:?}", v5);

    // let third: &i32 = &v5[2];
    // println!("The third element is {}", third);

    match v5.get(2) {   // get method returns an option, but this can be accessed in the 'Some'
        Some(third) => println!("The third element is {:?}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // this will panic
    let does_not_exist = v.get(1);
    println!("{:?}", does_not_exist);
*/
    let v = vec![10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));

    let v3 = dbg!(v.get(0..2));       // v3 is an Option, containing an array pointer
    println!("v3 is: {:?}",v3);

    let mut v1: Vec<i32> = Vec::new();    
    match v.get(0..2) {
        Some(v) => {                // this v is not the same v
            v1 = v.to_vec();        // v is an array not a Vec, so has to be converted
            println!("in Some scope, v is: {:?} and v1 is: {:?}", v, v1);
        }
        None => println!("There is no thing."),
    }

    println!("v is still {:?} and v1 is still: {:?}", v, v1);

    let s = dbg!([10, 40, 30]);   // s is an array, not a Vec
    let x = dbg!(s);              // x also becomes an array
    let mut y = dbg!(s.to_vec());     // but y becomes a Vec
    y.push(7);
    println!("s is array {:?} and x is array: {:?}, but y is Vec: {:?}", s, x, y);
}
