fn main() {
    // let x = 5;
    let mut x = 5;
    x = 6;

    const MAX_V: u16 = 42; // CONST!

    /* shadowing */
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    let spaces = "     ";
    // spaces = spaces.len();
    let spaces = spaces.len();
}
