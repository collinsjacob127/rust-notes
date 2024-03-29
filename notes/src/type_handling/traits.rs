/*
Name: Jacob Collins
Description:
Notes on traits, their uses, implementations, etc.
Last Edited: August XX 2022
*/

#[allow(unused_imports)]
use core::fmt::Display;

pub fn traits() {
    todo!();
}
// Rust checks the validity of all this at compile time so you don't
// actually have to run the code to tell you messed up, it just
// lets you know as you write it, more or less.

/*
Blanket trait implementation:
Implements a new trait for any type that implements a specified trait

impl<T: Display> ToString for T {
    // --snip--
}
*/

// ------- TRAIT RETURNS -------
// you can only return based on trait if there's only one type in your function to return
// The return type is arbitruary but must be known at compile time
#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// ------- TRAIT PARAMETERS -------
// where -> easy to read trait parameters
#[allow(dead_code, unused_variables)]
fn some_function<T, U>(x: T, y: U)
where
    T: Summary + Clone,
    U: Display,
{
    todo!()
}

// &impl Summary limits 'item' to types which implement summary
// + syntax to specify multiple necessary traits
#[allow(dead_code)]
fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news: {}", item.summarize());
}

/*
&impl is syntactic sugar for Trait-Bound:
    pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/

/*
forces all parameters of T to be the same type
pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {...}
*/

// ------- TRAIT DEFINITIONS & IMPLEMENTATIONS -------
// Orphan rule:
// you can't define a non-local trate (e.g. Display) on a non-local type (e.g. Vec)

// Conditionally implementing a type (cmp_display) depending on traits of a generic struct
#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Method defiition is only valid if type T of pair
// has both the Display and PartialOrd traits
#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Traditional definition of a trait
pub trait Summary {
    // Method that all types which implement Summary must
    // provide their own definition specific to the type:

    // method as signature:
    // REQUIRES IMPLEMENTATION
    fn summarize_author(&self) -> String;

    // method  with default:
    // This default happens to call the above method
    // Only the above method need be defined
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Overwriting default for custom value:
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!(" {}", self.author)
    }

    // Other methods required by summary if necessary:
    // ...
}

// Loading trait as default:
// impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
