use std::fmt;
use std::ops::Add;
use std::slice;

fn derefer_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32; // create an immutable raw pointer
    let r2 = &mut num as *mut i32; // create a mutable raw pointer

    let address = 0x12345usize;
    let r = address as *mut i32; // create a raw pointer to an arbitrary memory address

    // derefer the raw pointer using unsafe keywords
    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);

        // *r = 10;  // segment fault
        // println!("r is {}", *r);
    }
}

unsafe fn dangerous() {
    println!("this is an unsafe dangerous function")
}

fn call_unsafe_function_or_method() {
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // using the safe split_at_mut function
    let (a, b) = r.split_at_mut(3);

    println!("split 3 at a is {:?}", a);
    println!("split 3 at b is {:?}", b);

    // create save wrapper or abstraction to the unsafe code with an implementation of the function
    // that uses unsafe code in a safe way
    let (c, d) = my_raw_pointer_split_at_mut(&mut v[..], 4);

    println!("split 4 at c is {:?}", c);
    println!("split 4 at d is {:?}", d);

    // create a slice from an arbitrary memory location
    let address = 0x123456usize;
    let rr = address as *mut i32;
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(rr, 10000) };
}

fn my_raw_pointer_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])
    // borrowing the same slice twice will cause compile error
    let ptr = slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
// extern keyword for FFI(foreign function language
extern "C" {
    // CAPITAL C
    fn abs(input: i32) -> i32;
}

// call C from Rust
fn use_extern_function_to_call_extern_code() {
    unsafe {
        println!("Absolute value of -3 from c lib is {}", abs(-3));
    }
}

// call Rust from C and the usage of extern does not require unsafe
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from C!!!");
}

static HELLO_WORLD: &str = "hello world";

// const allowed to duplicate their data while static varible not and static variable allowed to be
// mutable (but not safe for multi-thread data reaces) while const not

static mut COUNTER: u32 = 0;

fn add_to_static_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn access_mutable_static_variable() {
    unsafe {
        println!("before added static count is {}", COUNTER);

        add_to_static_count(3);

        println!("after added static count is {}", COUNTER);
    }
}

// implement an unsafe trait
unsafe trait Foo {
    fn test() {}
}

unsafe impl Foo for i32 {
    fn test() {
        println!("This is an i32 implementation for unsafe trait named Foo");
    }
}

// access fileds of a Union

fn unsafe_rust() {
    derefer_raw_pointer();
    call_unsafe_function_or_method();
    use_extern_function_to_call_extern_code();
    access_mutable_static_variable();
    println!("static string is {}", HELLO_WORLD);
}

// specify placeholder types in trait definations with associated types
pub trait Iterator {
    type Item; // placeholder
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let it: u32 = 4;
        Some(it)
    }
}

// default generic type parameters and operator overloading

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// using the default Rhs type of Self
impl Add for Point {
    type Output = Point; // placeholder
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);

struct Meters(u32);

// set the Rhs type parameter instead of using the default of Self
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// fully qualified syntax for disambiguation of calling methods with the same name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is a Human of Pilot method ");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("This is a Human of Wizard method ");
    }
}

impl Human {
    fn fly(&self) {
        println!("This is a Human Self association function");
    }
}
// Associated Functions are part of traits don`t have a self parameter

trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

// using supertraits to require one traits functionality within another trait
trait OutlinePrint: fmt::Display {
    // OutlinePrint trait requires the Display trait
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct MyPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for MyPoint {}

impl fmt::Display for MyPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// using the Newtype pattern to implement external traits on external types

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// usint Newtype pattern for type safety and abstraction or encapsulation to hide implementation
// details

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>;
    fn flush(&mut self) -> Result<(), std::io::Error>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), std::io::Error>;
}

// make new type to reduce duplication
type ResultA<T> = std::result::Result<T, std::io::Error>;

pub trait AliasWrite {
    fn write(&mut self, buf: &[u8]) -> ResultA<usize>;
    fn flush(&mut self) -> ResultA<()>;
    fn write_all(&mut self, buf: &[u8]) -> ResultA<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> ResultA<()>;
}

// return Never type that never returns
fn bar() -> ! {
    panic!("hello")
}

// continue and panic! has a Never type
//
// trait MyOption<T = u32> {}
// impl<T> dyn MyOption<T> {
//     pub fn unwrap(t: Option<T>) -> Option<T> {
//         match t {
//             Some(_) => println!("Some type matched"),
//             None => panic!("called `Option::unwrap()` on a `None` value"), // panic! return a ! type
//         }
//     }
/* } */

fn advanced_types() {
    type Kilometers = i32; // i32 and new type Kilometers are the same type

    let x: i32 = 5;
    let y: Kilometers = 9;
    println!("x + y = {}", x + y);

    // tpye alias for verbose type
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("long type annotation examples"));

    fn takes_long_type(f: Thunk) {}
    fn return_long_type() -> Thunk {
        let g: Thunk = Box::new(|| println!("long type annotation examples"));
        g
    }

    // DST : Dynamical Sized Type
    let s1: &str = "hello rust";
    // let s2: str = "hello world";
    generic(5);
    generic_v2(&5);
}

fn generic<T: Sized>(t: T) {
    println!("This is a determined type size at compile time");
}

fn generic_v2<T: ?Sized>(t: &T) {
    //change T to &T or compile failed for undetermined type size at compile time
    println!("This is a undetermined type size at compile time");
}

fn advanced_traits() {
    let res: Point = Point { x: 5, y: 4 } + Point { x: 2, y: 3 };
    println!("point add result is {:?}", res);

    let res: Millimeters = Millimeters(100) + Meters(1);
    println!("100 mm add 1 m result is {:?} mm", res);

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    println!("A baby dog is {}", Dog::baby_name());
    // println!("A baby dog is {}", Animal::baby_name()); type annotations needed
    println!("A baby dog is {}", <Dog as Animal>::baby_name()); // <Type as Trait>::associated_fun()

    // let p = MyPoint { x: 5, y: 6 };
    let p: MyPoint = MyPoint { x: 5, y: 6 };
    println!("p is {}", p);
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("rust")]);
    println!("w = {}", w);
}

// function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

// fn keyword to annotate a function pointer TYPE while Fn is a TRAIT and function pointers
// implement all three of the closure traits(Fn, FnMut, FnOnce)
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// return closures
//
// use trait object to solve the closure return closure storage at compile time

// fn returns_closure() -> dyn Fn(i32) -> i32 {
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn advanced_function_and_closure() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);

    // use a closure defined inline
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!(
        "list_of_strings map convert using closure result is {:?}",
        list_of_strings
    );
    // using to_string function defined in ToString Trait
    let list_of_strings_v2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!(
        "list_of_strings map convert using named function pointer result is {:?}",
        list_of_strings_v2
    );

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    // Status::Value() is a initializer syntax just like a function call as a function pointer
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    for i in list_of_statuses {
        println!("status is {:?}", i);
    }
}

// macros
//1. variable number of parameter
//2. expaned at compile time such as implement a trait on a given type

// write a custom derive macro
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// macros
fn macros() {
    // let v: Vec<u32> = vec![1, 2, 3];
    Pancakes::hello_macro();
}

fn main() {
    unsafe_rust();
    advanced_traits();
    advanced_types();
    advanced_function_and_closure();
    macros();
}
