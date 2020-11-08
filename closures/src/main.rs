use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// user define Iterator for Counter
impl Iterator for Counter {
    type Item = u32; /* specify Item type*/

    /* impl next function */
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // type_infer_and_annotate();
    // let sim_user_spe = 3;
    // let sim_rand = 3;
    // generate_workout(sim_user_spe, sim_rand);
    // generate_workout_v2(sim_user_spe, sim_rand);
    // generate_workout_v3(sim_user_spe, sim_rand);
    // generate_workout_v4(sim_user_spe, sim_rand);

    // cap_env();
    // cap_env_failed();
    iter_test();
}

fn iter_test() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("value: {}", val);
    }
}

fn generate_workout_v4(intensity: u32, random_number: u32) {
    let mut exp_res = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("today, do {} pushups", exp_res.value(intensity));
        println!("next, do {} sitpus", exp_res.value(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("today, run for {} minutes", exp_res.value(intensity));
        }
    }
}

fn generate_workout_v3(intensity: u32, random_number: u32) {
    // let exp_closure = |num| {
    //
    let exp_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("today, do {} pushups", exp_closure(intensity));
        println!("next, do {} sitpus", exp_closure(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("today, run for {} minutes", exp_closure(intensity));
        }
    }
}

fn generate_workout_v2(intensity: u32, random_number: u32) {
    let res = sim_exp_calc(intensity);
    if intensity < 25 {
        println!("today, do {} pushups", res);
        println!("next, do {} sitpus", res);
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("today, run for {} minutes", res);
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("today, do {} pushups", sim_exp_calc(intensity));
        println!("next, do {} sitpus", sim_exp_calc(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("today, run for {} minutes", sim_exp_calc(intensity));
        }
    }
}

fn sim_exp_calc(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn type_infer_and_annotate() {
    let closure_1 = |x| x;
    let s = closure_1(String::from("hello"));
    // let n = closure_1(3);
    println!("s is {}", s);
}

fn cap_env() {
    let x = 5;
    let equ_to_x = |z: i32| -> bool { z == x };
    let y = 5;
    println!("equ_to_x(y) return {}", equ_to_x(y));
}

fn cap_env_failed() {
    let x = 5;
    let x_vec = vec![2, 3, 4];
    let equ_to_x = move |z: i32| -> bool { z == x };
    let equ_to_x_vec = move |z: &[i32]| -> bool { z == x_vec };
    let z = vec![2, 3, 4];
    println!("equ_to_x_vec(z) result is {}", equ_to_x_vec(&z));
    println!("can use x here for i32 not movable {:?}", x);
    // println!("can not use x_vec here {:?}", x_vec);
}

#[cfg(test)]
mod test_closures {
    use super::*;
    #[test]
    fn call_with_diff_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn iter_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iter_sum() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iter_reproduce() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 12,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn calling_next_dir() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iter_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}
