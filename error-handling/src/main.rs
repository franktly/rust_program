use std::cmp::Ordering;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// #![allow(unused)]
// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    // panic!("crash and burn");
    // nest_match();
    // error_only_match();
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // read_username_from_file().expect("error");
    // simple_read_username_from_file().unwrap();
    // sim_v3_read_username_from_file().expect("Error");
    // let f = File::open("hello.txt")?; // return Result and main return () will cause mismatch ERROR
    // return_enum_test();
    // let home: IpAddr = "127.0.0.1".parse().unwrap();
    // println!("home is {:?}", home);
    // guess_num();
    guess_struct();
}

fn guess_struct() {
    let g = Guess::new(10);
    println!("g value is {}", g.value());
}

fn guess_num() {
    loop {
        let sec_num = 13;
        let guess = String::from("12");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100");
            continue;
        }

        match guess.cmp(&sec_num) {
            Ordering::Less => {
                println!("less than {}", sec_num);
                break;
            }
            Ordering::Equal => {
                println!("equal {}", sec_num);
                break;
            }
            Ordering::Greater => {
                println!("greater {}", sec_num);
                break;
            }
        }
    }
}

fn return_enum_test() -> Result<(), io::Error> {
    let f = File::open("hello.txt")?;
    Ok(())
}

fn sim_v3_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn sim_v2_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn simple_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn nest_match() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("problem opening the file: {:?}", error);
            }
        },
    };
}

fn error_only_match() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem openning the file {:?}", error);
        }
    });
}
