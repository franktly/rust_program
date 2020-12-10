fn main() {
    // Ownership Rules
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped
    //

    let s = "hello";

    {
        let ss = "hello"; // ss is valid  from this point forward
                          // do stuff with ss
    } // this scope is now over, and ss is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
                 // println!("s1 is {}", s1);
    let s3 = s2.clone();
    println!("s2 is {}, s3 is {}", s2, s3);

    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);
    // types with Copy Trait
    // 1. All the integer types, such as u32,
    // 2. The Boolean type, bool. with values true and false
    // 3. All the floating point types, such as f64
    // 4. The character type, char.
    // 5. Tuples, if they only contain types that are also copy, for example, (i32,i32) is Copy,
    //    but (i32, String) is not.

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into function and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // but i32 is Copy, so it's okay to still use x afterward

    let s1 = gives_ownership(); // moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back function, which also move its return value into s3

    let ss = String::from("hello world");
    let (s4, len) = calc_len(ss);
    println!("{} len is {}", s4, len);

    // ordinary reference
    let ref_s = String::from("hello world");
    let len_ref = calc_len_ref(&ref_s);
    println!("{} len is {}", ref_s, len_ref);

    // mutable reference
    let mut s5 = String::from("hello");
    change_str(&mut s5);

    let mut s6 = String::from("hello");
    // let r1 = &mut s6; r1 and r2 multi-mutable the same refer and cause data race
    {
        let r1 = &mut s6;
    } // r1 goes out of scope, so we can make a new reference with no problem
    let r2 = &mut s6;

    // println!("r1 is {}", r1);
    println!("r2 is {}", r2);
    // data race occurs:
    // 1. Two or more pointers access the same data at the some time
    // 2. At least one of the pointers is being used to write to the data
    // 3. There's no mechanism being used to synchronize access to the data

    let mut s7 = String::from("hello");
    let r1 = &s7; // No problem
    let r2 = &s7; // No problem
                  // let r3 = &mut s7; // BIG PROBLEM
    println!("r1 is {}", r1);
    println!("r2 is {}", r2);

    let r3 = &mut s7; // no problem, mutable refer is last used with no other immutable reference
    println!("r3 is {}", r3);

    // let d = dangle();
    let d = no_dangle();

    // 1. At any given time, you can have either one mutalbe reference or any number of immutalbe
    //    references
    //    2. References must always be valid

    slice_test();
} // Here , x goes out of scope, then s ,  biz s's value was moved, nothing special happens
  // s3 goes out of scope and is dropped
  // s2 goes out of scope but was moved, so nothing happens
  // s1 goes out of scope and is dropped

fn takes_ownership(some_str: String) {
    // some_str comes int scope
    println!("{}", some_str);
} // Here, some_str goes out of scope and `drop` si callled and memory is freed

fn makes_copy(some_int: i32) {
    // some_integer comes into scope
    println!("{}", some_int);
} // Here, some_int goes out of scope. nothing special happens

// some_str is returned and moves out to the calling function
fn gives_ownership() -> String {
    let some_str = String::from("hello"); // some_str comes into scope
    some_str // move its return value into function that calls it
}

// take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calc_len(s: String) -> (String, usize) {
    // s is moved into calc_len function
    let len = s.len();
    (s, len) //s,len is moved into funtion that calls it
}

fn calc_len_ref(s: &String) -> usize {
    s.len()
} // Here, s goeos out of scope, but bz it doesnot have ownership of what it refers to, nothing happens

fn change_str(some_str: &mut String) {
    some_str.push_str(", world");
}

// fn dangle() -> &String {
// return s a reference to String
// let s = String::from("hello"); // s is a new String
// &s // returns a reference to the String, s
// } // Here, s goes  out of scope, and is dropped, Its memory goes away , Danger!!!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn slice_test() {
    let mut s = String::from("hello world");

    let word = first_word(&s); //word will get the value 5;
    println!("first word tail index of {} is {}", s, word);
    s.clear(); // thie empties the String, making it equal to ""

    // word still has the value 5 here, but there is no more Strint that

    let mut ss = String::from("hello world");
    let f = first_word_slice(&ss); // immutable borrow
                                   // ss.clear(); mutable borrow
    println!("first word implemented by slice of string {} is {}", ss, f); // immutable borrow later used here

    let f1 = first_word_slice_str(&ss[..]);
    println!(
        "first word implemented by str slice of string {} is {}",
        ss, f1
    ); // immutable borrow later used here

    let sss = "hello"; // sss is a immutable const &str type
    let f2 = first_word_slice_str(sss);
    println!(
        "first word implemented by str slice of string {} is {}",
        sss, f2
    ); //

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("first world of {}  is {}", s, hello);
    println!("second world of {}  is {}", s, world);

    let len = s.len();

    let slice1 = &[3..len];
    println!("slice1 of {} is {:?}", s, slice1);

    let slice2 = &[3..];
    println!("slice2 of {} is {:?}", s, slice2);

    let slice3 = &[0..len];
    println!("slice3 of {} is {:?}", s, slice3);

    let slice4 = &[..];
    println!("slice4 of {} is {:?}", s, slice4);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..4];
    for i in slice.iter() {
        println!("cur slice is {}", i);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
