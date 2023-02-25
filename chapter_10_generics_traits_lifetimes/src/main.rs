use std::cmp::*;

/* Need to restrict T to only include types that implement > */
fn largest<T: PartialOrd>(list: &[T]) -> &T {
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

/* Use a trait bound to conditionally associate traits */
impl<T: Summary> Summary2 for Point<T> {
    fn summarize2(&self) -> () {
        println!("Implements Summary2 because implements summary 1");
    }
}

/* Use a trait bound to define methods for all types implementing a Trait(s) - Blanket implementations */
impl<T: Summary> Point<T> {
    fn new_method(&self) {
        println!("Only implement for T's implementing summary");
    }
}

/* Prevent dangling references with lifetimes */
/* below produces dangling reference to x - x dropped while still being referenced by r.
     
   fn main() {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }


 * lifetime annotated 
    fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

    the size of the lifetime 'b (x) is smaller than that of 'a (r)
 */

/* Corrected lifetime */
fn lifetime() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

/* x and y parameters require lifetime annotations for the following reasons
 *
 * The compiler can't tell which reference will be returned at compile time. This is required to ensure that the referenced value remains valid.
 * 
 * Compiler cannot infer the concrete values that will be passed as argument so it cannot determine
 * which branch of the if.. else statement will be processed. 
 * 
 * The Compiler also cannot infer the lifetimes as the concrete lifetime of the references passed as
 * arguments may not be known. */
fn longest<'a>(x: & 'a str, y: & 'a str) -> & 'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

/* Lifetime annotations for structs */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/* Generic type parameters, trait bounds, and lifetimes together */
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
         y
    }
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
        "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}",
    tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Breaking!"),
        location: String::from("somewhere"),
        author: String::from("John Doe"),
        content: String::from("testing")
    };
    let p = Point {
        x: article
    };
    p.summarize2();
    let p1 = Point {
        x: tweet
    };
    p1.new_method();

    /* Generic lifetimes */
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    /* Lifetime annotation stuff
    { //'a
        let string1 = String::from("long string is long");
        let result;
        { //'b
            let string2 =
            String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}",result);
    }

    */
    /* Movin println! into the inner scope or moving string4 out sould satisfy the borrow checker /
     * lifetime annotations */
    let string3 = String::from("long string is long");
    let string4 = String::from("xyz");
    let result;
    {
        result = longest(string3.as_str(), string4.as_str());
    }
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    /* first call to .next() yields segment 0 delimited by the pattern provided to .split() */
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part)
}
