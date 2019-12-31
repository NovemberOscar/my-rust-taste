fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&s);

    println!("Length of {} is {}", s, len);

    //    change(&s)
    change(&mut s);

    let r1 = &s;
    let r2 = &s;
    //    let mr1 = &mut s;
    //    println!("{}, {}, and {}", r1, r2, mr1);

    let r3 = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//fn change(str: &String) {
fn change(str: &mut String) {
    str.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
