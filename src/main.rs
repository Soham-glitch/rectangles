#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(10);
    // Check if rect1 can hold rect2 by comparing dimensions
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // Check if rect1 can hold rect3 by comparing dimensions
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("square value is {:?}",square);
}
