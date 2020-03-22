fn largest_i32(ls: &[i32]) -> i32 {
    let mut largest = ls[0];

    for &e in ls.iter() {
        if e > largest {
            largest = e;
        }
    }

    largest
}

fn largest_char(ls: &[char]) -> char {
    let mut largest = ls[0];

    for &e in ls.iter() {
        if e > largest {
            largest = e;
        }
    }

    largest
}

//fn largest<T>(ls: &[T]) -> T {
//    let mut largest = ls[0];
//
//    for &e in ls.iter() {
//        if e > largest {
//            largest = e;
//        }
//    }
//
//    largest
//}

struct Point<T> {
    x: T,
    y: T,
}

struct NewPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T, U> NewPoint<T, U> {
    fn mixup<V, W>(self, other: NewPoint<V, W>) -> NewPoint<T, W> {
        NewPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&chars);
    println!("The largest char is {}", result);

    //    let numbers = vec![34, 50, 25, 100, 65];
    //
    //    let result = largest(&numbers);
    //    println!("The largest number is {}", result);
    //
    //    let chars = vec!['y', 'm', 'a', 'q'];
    //
    //    let result = largest(&chars);
    //    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //    let wont_work = Point { x: 5, y: 4.0 };
    let integer_and_float = NewPoint { x: 5, y: 4.0 };

    println!("p.x = {}", integer.x());

    let mixed = (NewPoint { x: 5, y: 4.0 }).mixup(NewPoint { x: 4, y: 5.0 });
    println!("p3.x = {}, p3.y = {}", mixed.x, mixed.y);
}
