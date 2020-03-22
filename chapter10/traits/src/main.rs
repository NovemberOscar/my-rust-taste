mod summarize {
    pub trait Summarizable {
        fn summary(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarizable for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub trait SummarizableWithDefault {
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }

    //    pub fn notify<T: Summarizable>(item: T) {
    //        println!("Breaking news! {}", item.summary());
    //    }

    pub fn notify<T>(item: T)
    where
        T: Summarizable,
    {
        println!("Breaking news! {}", item.summary());
    }
}

fn largest<T: PartialOrd>(ls: &[T]) -> &T {
    let mut largest = &ls[0];

    for e in ls.iter() {
        if e > largest {
            largest = e;
        }
    }

    largest
}

fn main() {
    use summarize::*;

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    notify(tweet);

    let numbers: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars: Vec<char> = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
