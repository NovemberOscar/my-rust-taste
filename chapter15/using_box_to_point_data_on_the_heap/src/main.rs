use crate::ConsList::{Cons, Nil};

enum ConsList {
    Cons(i32, ConsList),
    Nil,
}

fn main() {
    let cons_ls = Cons(1, Cons(2, Cons(3, Nil)));
}
