fn f1() {
    println!("Another hello, world!");
}

fn f2(x: i32) {
    println!("Value of x: {}", x);
}

fn f3(x: i32, y: i32) {
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
}

fn stmt_and_exp() {
    let y = 6;
    // let x = (let y = 6);
    let x = {
        let y = 7;
        y + 1
    };
    println!("Value of x: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    f1();
    f2(5);
    f3(6, 7);
    stmt_and_exp();
    println!("five: Return value is: {}", five());
    println!(
        "plus_one: given value was {} and return value is {}",
        5,
        plus_one(5)
    );
}
