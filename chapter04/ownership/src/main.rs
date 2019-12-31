fn main() {
    // s is not valid now
    {
        let s = "hello"; // s is valid from this point forward
    } // s is not valid now

    {
        let mut s = String::from("hello"); // s is valid again
        s.push_str(", world!"); // do something with s
        println!("{}", s);
    } // s is not valid now

    // println!("{}", s); <- cannot find value `s` in this scope

    {
        let x = 5; // x = 5
        let y = x; // y = 5

        println!("{} {}", x, y);

        let s1 = String::from("hello");
        let s2 = s1;

        println!("{}", s2);
        // println!("{}", s1); <- borrow of moved value: `s1`
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{} {}", s1, s2)
    }

    {
        let s = String::from("hello");
        takes_ownership(s);
        let x = 5;
        makes_copy(x);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}