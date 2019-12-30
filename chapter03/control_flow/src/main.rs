fn main() {
    let number = 3;
    // let number = 7;

    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.")
    }

    // if number {
    if number != 0 {
        println!("condition was true");
    }

    let cond = true;

    let number = if cond {
        5
    } else {
        // "six"
        6
    };
    println!("Value of number: {}", number);

    // loop {
    //     println!("again!");
    // }

    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("The result is {}", result);

    let arr = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("Element of arr: {}", arr[i]);
        i += 1;
    }

    for i in arr.iter() {
        println!("Element of arr: {}", i);
    }

    for n in (1..4).rev() {
        println!("{}", n);
    }
    println!("Launch!");
}
