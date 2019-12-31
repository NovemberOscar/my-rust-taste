fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    //    s.clear();

    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..];
}

//fn first_word(s: &String) -> usize {
fn first_word(s: &String) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            //            return i;
            return &s[0..i];
        }
    }
    //    s.len()
    &s[..]
}
