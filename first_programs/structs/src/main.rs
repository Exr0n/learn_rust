#[derive(Debug)]
struct Rectangle {
        width: u32,
        height: u32
    }

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("Rectangle is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let smaller = Rectangle::square(20);
    println!("Smaller can hold larger: {}", smaller.can_hold(&rect));
    println!("Larger can hold smaller: {}", rect.can_hold(&smaller));
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}