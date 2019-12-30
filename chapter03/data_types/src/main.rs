fn main() {
    // let y = "42".parse().expect("Not a number.");
    let x: u32 = "42".parse().expect("Not a number.");

    /* Scalar Types */
    let i: i8 = 127;
    let i: u8 = 255;
    let i: i32 = 2147483647;
    let i: u32 = 4294967295;

    let i = 10_000;
    let i = 0xff;
    let i = 0o77;
    let i = 0b1111_1111;
    let i: u8 = b'A';

    let f = 2.0; // f64
    let f: f32 = 2.0; // f32

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 2.0;
    let remainder = 43 % 5;

    let b = true;
    let b: bool = false;

    let c = 'z';
    let c = '🤖';

    /* Compound Types */
    let tup: (i32, f64, u8) = (500, 4.2, 1);
    let tup = (500, 4.2, 1);
    let (x, y, z) = tup;
    let index_zero = tup.0;

    let arr = [1, 2, 3, 4, 5];
    // let arr = [1, 2, 3.2, "a"];

    let first = arr[0];
}
