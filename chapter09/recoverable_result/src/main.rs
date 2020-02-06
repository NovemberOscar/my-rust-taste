use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn open_hello() {
    let f = File::open("hello.txt");
    //    let f: u32 = File::open("hello.txt");
}

fn open_hello_with_handling() {
    let f = File::open("hello.txt");

    //    let f: Option<File> = match f {
    //        Ok(file) => Some(file),
    //        Err(_) => None,
    //    };

    let f = match f {
        Ok(file) => file,
        Err(_) => panic!("Not Found."),
    };
}

fn open_hello_or_create_hello() {
    let f = File::open("hello.txt");

    //    let s = String::from("hello");
    //    let r1 = &s;
    //    let ref r1 = s;

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            // http://xion.io/post/code/rust-patterns-ref.html
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn open_hello_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn open_hello_expect() {
    let f = File::open("hello.txt").expect("Not Found");
}

fn open_hello_or_propagate() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn open_hello_with_propagate_shortcut() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    //    open_hello();
    //    open_hello_with_handling();
    //    open_hello_or_create_hello();
    //    open_hello_unwrap();
    //    open_hello_expect();
    match open_hello_or_propagate() {
        Ok(_) => println!("File opened"),
        Err(_) => println!("File not found"),
    }
}
