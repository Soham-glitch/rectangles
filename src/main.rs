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
    println!("square value is {:?}", square);

    // Use the IP address constants
    println!("Home IP: {:?}", HOME);
    println!("Loopback IP: {:?}", LOOPBACK);

    // Option enum examples
    let some_number = Some(5);
    let some_character = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_character: {:?}", some_character);
    println!("absent_number: {:?}", absent_number);
}

// Enum examples
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(&'static str),
}

const HOME: IpAddr = IpAddr::V4(127, 0, 0, 1);
const LOOPBACK: IpAddr = IpAddr::V6("::1");
