use crate::ConsList::{Cons, Nil};

enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

fn main() {
    let cons_ls = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
