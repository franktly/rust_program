// generic type
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the larger num is {}", self.x);
        } else {
            println!("the larger num is {}", self.y);
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub trait SummaryWithDefault {
    fn summarize_with_def(&self) -> String {
        String::from("(Read more ...)")
    }
}

pub struct News {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl SummaryWithDefault for News {}

impl Summary for News {
    // fn summarize(&self) -> String {
    // format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    // format!("{} : {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@@@{}", self.username)
    }
}

impl SummaryWithDefault for Tweet {
    fn summarize_with_def(&self) -> String {
        format!("{} says: {}", self.username, self.content)
    }
}

//
// impl<T: Display> ToString for T {}

fn condition_method_trait() {
    let p = Pair { x: 10, y: 30 };
    let n = Pair::new(20, 40);
    p.cmp_display();
    n.cmp_display();
}

fn basic_trait_test() {
    let tweet = Tweet {
        username: String::from("frank"),
        content: String::from("hello world , welcome to Rust world"),
        reply: false,
        retweet: false,
    };

    println!("tweeter : {}", tweet.summarize());
    println!("tweeter : {}", tweet.summarize_with_def());

    let news = News {
        headline: String::from("hello"),
        location: String::from("Beijing"),
        author: String::from("frank"),
        content: String::from("Beijing welcome you"),
    };
    println!("news: {}", news.summarize());
    println!("news : {}", news.summarize_with_def());
}

pub fn notify(item: &impl Summary) {
    println!("Notify Breaking news! {}", item.summarize());
}

pub fn combine_notify(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Combine Notify Breaking news! {} AND {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!(
        "Trait Bound Syntax Notify Breaking news! {}",
        item.summarize()
    );
}

pub fn combine_notify_trait_bound<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Combined Trait Bound Syntax Notify Breaking news! {} AND {}",
        item1.summarize(),
        item2.summarize()
    )
}

// pub fn notify_multi(item: &(impl Summary + Display)) {}
// pub fn notify_multi<T: Summary + Display>(item: &T) {}
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
// T: Display + Clone,
// U: Clone + Debug,
// { }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime method elision
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// lifetime method elision
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, ann: &str) -> &str {
        println!("attention: {}", ann);
        self.part
    }
}

fn main() {
    // generic function
    let num_list = vec![43, 323, 40, 56, 100];
    let result = mid(&num_list);
    println!("middle num is {}", result);

    let char_list = vec!['a', 'c', 's', 'q', 'm'];
    let result = mid(&char_list);
    println!("middle char is {}", result);

    // generic struct

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer is ({}, {})", integer.x, integer.y);
    println!("float is ({}, {})", float.x, float.y);
    // let wont_work = Point { x: 5, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "integer and float is ({}, {})",
        integer_and_float.x, integer_and_float.y
    );
    let p = Point { x: 10.0, y: 20.0 };
    println!("p.x is {}", p.x());
    println!("distance_from_origin is {}", p.distance_from_origin());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 {
        x: "hello",
        y: "world",
    };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    basic_trait_test();

    let tweet = Tweet {
        username: String::from("frank"),
        content: String::from("hello world , welcome to Rust world"),
        reply: false,
        retweet: false,
    };

    let news = News {
        headline: String::from("Trump's tweeter acount is hacked by hacker"),
        location: String::from("BBC"),
        author: String::from("frank"),
        content: String::from(
            "Trump's tweeter is hacked and catch America president safe guard 's eyes",
        ),
    };

    // traits as parameters
    notify(&tweet);
    notify_trait_bound(&tweet);
    combine_notify(&tweet, &news);
    combine_notify_trait_bound(&news, &news);

    largest_impl();
    condition_method_trait();
    prevent_dangling();
    anno_lifetime();
    struct_hold_ref();

    // static lifetime
    let s: &'static str = "i have a static lifetime.";

    let str1 = String::from("h1");
    let str2 = String::from("h2");
    let str3 = String::from("h3");

    let ret = longest_and_announcement(&str1, &str2, str3);
    println!("ret is {}", ret);
}

fn longest_and_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
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

fn first_word(s: &str) -> &str {
    "hello"
}

fn first_word_v2<'a>(s: &'a str) -> &str {
    "hellov2"
}

fn first_word_v3<'a>(s: &'a str) -> &'a str {
    "hellov3"
}

fn struct_hold_ref() {
    let novel = String::from("call.from.frank");
    let first_sen = novel.split('.').next().expect("could not find a '");
    let i = ImportantExcerpt { part: first_sen };
}

// fn longest_v2<'a>(x: &str, y: &str) -> &'a str {
// let result = String::from("really long string");
// result.as_str()
// }

fn anno_lifetime() {
    let str1 = String::from("abcdef");
    // let result;
    {
        let str2 = String::from("xyz");
        let res = longest(&str1, &str2);
        // result = longest(&str1, &str2);
        println!("longest: {}", res);
    }
    // println!("longest: {}", result);
}

fn prevent_dangling() {
    let r: i32 = 0;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("frank"),
        content: String::from("hello world"),
        reply: false,
        retweet: false,
    }
}

fn largest_impl() {
    let num_list = vec![34, 50, 25, 100, 65];
    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'c', 'd', 'm', 'n', 'p'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn mid<T>(list: &[T]) -> &T {
    let len = list.len();
    &list[len / 2]
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut ret = list[0];

    for &item in list {
        if item > ret {
            ret = item;
        }
    }

    ret
}
