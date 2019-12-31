#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.length > r.length && self.width > r.width
    }

    fn new(size: u32) -> Rectangle {
        Rectangle {length: size, width: size}
    }
}

fn main() {
    let r1 = Rectangle {length: 50, width: 30};
    println!("r1 is {:?}", r1);
    println!("area of r1 is {}", r1.area());
    let r2 = Rectangle { length: 30, width: 20 };
    println!("r1 can hold r2?: {}", r1.can_hold(&r2));
    let r3 = Rectangle::new(12);
    println!("r3 is {:?}", r3);
}
