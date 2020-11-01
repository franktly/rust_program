fn main() {
    let s = String::from("hello world");
    let fw = first_word(&s);
    // s.clear();
    println!("s is {}", s);
    println!("first word is {}", fw);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello is {}", hello);
    println!("world is {}", world);

    let slice = &s[0..2];
    println!("slice is {}", slice);
    let slice = &s[3..];
    println!("slice is {}", slice);

    let len = s.len();
    let slice = &s[0..len];
    println!("slice is {}", slice);
    let slice = &s[..];
    println!("slice is {}", slice);

    let slice = first_word_slice(&s);
    println!("slice is {}", slice);

    let my_str = String::from("hello world");

    // first word
    let word = first_word_slice_s(&my_str[..]);
    println!("first word is {}", word);

    let my_str_lit = "hello world";

    let word = first_word_slice_s(&my_str_lit[..]);
    println!("first word is {}", word);

    let word = first_word_slice_s(my_str_lit);
    println!("first word is {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &[1..3];

    for i in a.iter() {
        println!("origin is {}", i);
    }

    // for i in [1..3].step_by(1){
    // println!("slice is {}", i);
    // }
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

fn first_word_slice_s(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
