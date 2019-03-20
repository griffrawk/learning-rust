// rust code to play with structs, methods, debugging, modules, use, and unit tests
// TODO some more todos

mod rectangle;                // load external rust module rectangle.rs. this also brings in its
                              // submodule amiprivate, defined as public
mod moddy {                   // inline module, not in the scope of main unless explicit
    pub fn moddy_print() {    // public fn use as moddy::moddy_print()
        println!("moddy mod mod");
        wibble_print();
    }
    fn wibble_print() {       // private function within inline module
        println!("wibbley woo!");
    }
}

use rectangle::*;             // brings all in rectangle into current scope
use moddy::moddy_print;       // brings the inline module moddy into current scope
use amiprivate::*;

fn main() {
    moddy_print();

    amiprivate_func();

// TODO add some more rectangles
    let rect1 = Rectangle { width: 32321345636320.0, height: 5234541234140.0 };
    let rect2 = Rectangle { width: 10.0, height: 40.0 };
    let rect3 = Rectangle { width: 60.0, height: 45.0 };

// using the derive(Debug). # is pretty print, x is lowercase hex, ? is Debug trait
// see docs for std::fmt
    // println!("rect1 in hex is {:#x?}", rect1); // hex doesn't work for float ofc!
    println!("rect1 in dec is {:#?}", rect1);

    /*

what a todo

*/
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(32.0);

    // TODO in master
    println!("sq is {:#?}", sq);
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
}
