use std::fmt::Display;

struct CustomSmartPointer<T: Display>(T);

impl<T: Display> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.0);
    }
}

fn main() {
    let _c = CustomSmartPointer(String::from("my stuff"));
    let _d = CustomSmartPointer("other stuff");
    println!("CustomSmartPointers created.");
}
