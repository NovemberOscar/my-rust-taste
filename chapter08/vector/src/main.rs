#[allow(dead_code, unused_variables)]
fn new_vector() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
}

#[allow(dead_code, unused_variables)]
fn vector_scope() {
    {
        let v = vec![1, 2, 3];
    }
    //    println!("{}", v);
}

#[allow(dead_code, unused_variables)]
fn reading_vector() {
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    match third {
        Some(_) => {
            println!("Reachable element");
        }
        None => {
            println!("Unreachable element");
        }
    }
    //    let does_not_exist = &v[100];
}

#[allow(dead_code, unused_variables, unused_mut)]
fn invalid_reference() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //    v.push(6);

    println!("{}", first);
}

#[allow(dead_code, unused_variables)]
fn loop_vector() {
    let v = vec![1, 2, 3, 4];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        *i *= 10;
    }
}

#[allow(dead_code, unused_variables)]
fn vector_with_enum() {
    #[derive(Debug)]
    enum Number {
        Int(i32),
        Float(f64),
    }

    let v = vec![Number::Int(3), Number::Float(2.0)];

    for i in &v {
        println!("{:?}", i);
    }
}

#[allow(dead_code, unused_variables)]
fn main() {
    //    new_vector();
    //    vector_scope();
    //    reading_vector();
    //    invalid_reference();
    //    loop_vector();
    //    vector_with_enum();
}
