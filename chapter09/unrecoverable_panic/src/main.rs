fn just_panic() {
    panic!("crash and burn");
}

fn try_read_vec() {
    let v = vec![1, 2, 3];

    v[99];
}

fn main() {
    //    just_panic();
    try_read_vec();
}
