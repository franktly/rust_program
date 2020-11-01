#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_four(a: i32) -> i32 {
    a + 4
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        if value < 1 {
            panic!("Guess value must greater than 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must less than 100, got {}", value);
        }

        Guess { value }
    }
}

fn print_and_return_10(a: i32) -> i32 {
    println!("i got the value {}", a);
    10
}

pub mod for_integration {
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    pub fn mul_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn internal_mul(a: i32) -> i32 {
    a * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inter_test() {
        assert_eq!(4, internal_mul(2));
    }
    #[test]
    #[ignore]
    fn expensive_test() {}

    #[test]
    fn print_return_pass() {
        let v = print_and_return_10(4);
        assert_eq!(10, v);
    }

    #[test]
    #[ignore]
    fn print_return_fail() {
        let v = print_and_return_10(4);
        assert_eq!(6, v);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 not equal 4"))
        }
    }

    #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(500);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_add_four() {
        assert_ne!(5, add_four(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
