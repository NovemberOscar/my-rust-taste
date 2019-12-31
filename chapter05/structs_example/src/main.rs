#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area(r: &Rectangle) -> u32 {
    r.length * r.width
}

fn main() {
    let r1 = Rectangle {length: 50, width: 30};
    println!("{:#?}", r1);
    println!("{}", area(&r1));
}
