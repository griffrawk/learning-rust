pub mod amiprivate;  // module belongs to rectangle, but is made pub so main can use it

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

// ooo! methods!
impl Rectangle {
    // methods work on self
    // e.g. rect1.area()
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // functions don't work on self, but can be used to instantiate a struct
    // e.g. let sq = Rectangle::square(8)
    pub fn square(size: f32) -> Rectangle {
        amiprivate::amiprivate_func();
        Rectangle { width: size, height: size }
    }
}

// Unit tests
#[test]
fn test1() {
    let rect1 = Rectangle { width: 30.0, height: 50.0 };
    assert_eq!(rect1.area(), 1500.0);
}

#[test]
fn test2() {
    let rect1 = Rectangle { width: 3.0, height: 5.0 };
    assert_eq!(rect1.area(), 15.0, "Oh no what a disaster. The laws of the universe have been violated.");
}
