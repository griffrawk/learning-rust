#[allow(dead_code)]
fn main() {
    println!("Traits");

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {         // default summarize fn for trait
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Book {
        pub title: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for Book {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.title, self.author, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }     // NewsArticle uses the summarize default for trait, but has to define a summarize_author

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // trait as argument. the fn is not tied (impl'd) to specific struct but will only accept
    // structs impl Summary
    // pub fn notify(item: impl Summary) {                  // the sugary version or ...
    pub fn notify<T: Summary>(item: T) {                    // as a trait bound
        println!("Breaking news! {}", item.summarize());    // item.summarize can be diff depp on impl
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let book = Book {
        title: String::from("Big book of stories"),
        author: String::from("AA Million"),
        content: String::from("Some content")
    };
    println!("1 new book: {}", book.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    // use the notify fn, which only works for structs impl Summary trait
    notify(book);
    notify(tweet);
    notify(article);


}
