use std::cmp::PartialOrd;

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}  

struct Point<T> {
    x: T, 
}

/* Implemented for every type */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/* Implemented for only Point<i32> instances */
impl Point<i32> {
    fn convert(&mut self) -> () {
        self.x *= -1;
        
    }
}

/* Traits */
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    } 
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
       format!("{}:i {}", self.username, self.content)
    }
}

/* Default traits */
trait Summary2 {
    fn summarize2(&self) -> () {
        println!("This is a default trait");
    }
}

/* Use the default for NewsArticle */
impl Summary2 for NewsArticle {}

/* Override the default for Tweet */
impl Summary2 for Tweet {
    fn summarize2(&self) -> () {
        println!("Overrided default implementation of Summary2 trait");
    }
}

/* default impementations can call other non-default methods */
trait Summary3 {
    fn summarize3(&self) {
       println!("{}", self.non_default_method()); 
    }

    fn non_default_method(&self) -> String {
        String::from("Non-default")
    }
}

/* Defining a function with a trait as parameter */
/* only accepts types for item that implement the Summary trait */
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());    
}

/* above in trait bound syntax */
fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

/* two potentially different types with the same trait implementation */
fn notify3 (item1: & impl Summary, item2: & impl Summary) {
    
}

/* The above in trait bound syntax */
fn notify4<T,S: Summary> (item1: &T, item2: &S) {

}

/* two of the same types with the same trait implementation */
fn notify5<T: Summary>(item1: &T, item2: &T) {

}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    let tweet = Tweet {
        username:
        String::from("horse_ebooks"),
        content: String::from(
        "of course, as you probably
        already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}",
    tweet.summarize());
}
