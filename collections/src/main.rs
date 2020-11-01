use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    // Vec new macro vec! and auto infer (default i32 type
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // v.get(index) reurns an Option<&T> type
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element!!!"),
    }

    // vector out of range process
    // let does_not_exit = &v[100]; // program panicking
    let does_not_exit = v.get(100); // return None
    println!("v.get(100) result is {:?}", does_not_exit);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // mutable for first is immutable and may cause Vec memory realloc and invalid memory refer for first variant
    println!("The first element if {}", first);

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 23];
    for i in &mut v {
        *i += 50; // * derefer &
    }

    for i in &v {
        println!("add 50 is {}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // use the same type of enum to store diffrent type of value by Vec although does not support
    // originally
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.1),
    ];

    let mut s = String::new();
    println!("s is {}", s);
    let data = "initial contents";
    println!("data is {}", data);
    let s = data.to_string();
    println!("s is {}", s);

    let s = "initial contents".to_string();
    println!("s is {}", s);
    let s = String::from("initial contents");
    println!("s is {}", s);

    let hello = String::from("hello");
    println!("hello origin English is {}", hello);
    let hello = String::from("你好");
    println!("hello translated in Chinese is {}", hello);

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // not taking owership but string slice
    println!("s is {}", s);
    println!("s2 is {}", s2);
    s.push('A'); // push single char
    println!("s is {}", s);

    let s1 = String::from("hello ");
    let s2 = String::from(" world");
    let s3 = s1 + &s2; // s1 is moved out, s2 &String defer coericion to &str by rust implicitly
                       // println!("s1 is {}, s2 is {}, s3 is {}", s1, s2, s3);
    println!("s2 is {}, s3 is {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // let ss = s[0];  String not support integer index for avoiding Unicode char misunderstanding
    // and retrive O(1) time complex is too hard
    println!("hello len is {}", "hello".len());
    println!("你好 len is {}", "你好".len());
    let origin = "hello";
    let nihao = "你好";

    // use string slice to index String or str
    println!("hello 2 is {}", &origin[0..2]);
    // println!("nihao 2 is {}", &nihao[0..2]); // will cause panick

    // use chars to iterate over Strings
    for c in "你好啊".chars() {
        println!("{}", c);
    }

    for c in "你好啊".bytes() {
        println!("{}", c);
    }

    for c in "nihao".chars() {
        println!("{}", c);
    }

    for c in "nihao".bytes() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    println!("scores is {:#?}", scores);

    // collect two vector to hashmap by collect function
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
        String::from("Red"),
        String::from("Green"),
    ];

    let init_scores = vec![10, 20, 20, 30];

    let scores: HashMap<_, _> = teams.into_iter().zip(init_scores.into_iter()).collect(); // type auto infer
    println!("collected scores is {:#?}", scores);

    let field_name = String::from("color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map is {:#?}", map);
    // println!("name is {}, value is {}", field_name, field_value); // field_name and field_value
    // is moved out
    //
    let tn = String::from("Blue");
    let sc = scores.get(&tn); // return Option<&T>
    println!("sc is {:?} ", sc);

    // retrive HashMap via for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update HashMap

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("override result is {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50); // entry return a mutable reference of Entry enum : exit and not insert
    scores.entry(String::from("Red")).or_insert(30); // not exit and insert
    println!("entry result is {:?}", scores);

    // update a value based on the old value
    let text = "hello world awesome world hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let cnt = map.entry(word).or_insert(0); // entry return a mutable reference to the value of this key and update after plus one
        *cnt += 1; // derefere
    }

    println!("word repeat stastic result is {:#?}", map);
}
