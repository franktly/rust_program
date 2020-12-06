fn if_let_pattern_test() {
    let tmp_str = String::from("red");
    let fav_color: Option<&str> = Some(&tmp_str);
    // let fav_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple has the background color");
        } else {
            println!("Using orange has the background color");
        }
    } else {
        println!("Using blue has the background color");
    }
}

fn while_let_pattern_test() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop_pattern_test() {
    let v = vec!['a', 'b', 'c'];
    for (index, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, index);
    }
}

fn let_destruction_pattern_test() {
    let x = 5;
    println!("x is {}", x);

    let (a, b, c) = (1, 2, 3);
    println!("a,b,c = ({},{},{})", a, b, c);

    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current pos is ({}, {})", x, y);
}

fn refutable_irrefutable_test() {
    // let Some(x) = some_option_value;
    if let x = 5 {
        println!("{}", x);
    }
}

fn pattern_syntax_test() {
    // match literials
    let x = 8;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 | 5 => println!("four or five"),
        6..=10 => println!("six to ten"),
        _ => println!("anything"),
    }

    let w = 'c';
    match w {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // match named varibles
    let y = Some(5);
    let z = 10;

    match y {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, z = {:?}", z),
        _ => println!("Default case, y = {:?}", y),
    }
    println!("at the end: y = {:?}, z = {:?}", y, z);

    // destructing to break apart values

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    println!("breaking apart from Point and a = {}, b = {}", a, b);
    println!("breaking apart from Point and x = {}, y = {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    enum_test();

    // destructuring mix structs and  tuples
    let ((feet, inches), Point { x, y }) = ((2, 10), Point { x: 3, y: 5 });
    println!(
        "feet is {}, inches is {}, point is ({}, {})",
        feet, inches, x, y
    );

    foo(3, 4);

    // let mut v_1 = Some(5);
    let mut v_1 = None;
    let v_2 = Some(10);

    match (v_1, v_2) {
        (Some(_), Some(_)) => {
            println!("can not override an existing customized value");
        }
        _ => {
            v_1 = v_2;
        }
    }
    println!("v1 is {:?}", v_1);

    let numbers = (2, 4, 6, 8, 10);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers in first and last is : {}, {}", first, last);
        }
    }

    // difference bt '_' and '_variant' (bind or not bind)

    let ss = Some(String::from("hello"));
    let ss2 = Some(String::from("hello"));
    if let Some(_s) = ss {
        println!("found a string {}", _s);
    }

    if let Some(_) = ss2 {
        println!("found a string");
    }

    // println!("ss = {:?}", ss); moved to _s and will compile failed
    println!("ss2 = {:?}", ss2); // _ not moved and binded to _

    // ignore remaining parts of a value with ..
    struct MyPoint {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = MyPoint { x: 10, y: 0, z: 0 };

    match origin {
        MyPoint { x, .. } => println!("my point matched  x is {}", x),
    }

    // match guards
    // let num = Some(4);
    let num = Some(8);
    match num {
        Some(x) if x < 5 => println!("less than five {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    match_guard_with_outer_var();

    // binding test
    enum MyMessage {
        Hello { id: i32 },
    }

    let my_msg = MyMessage::Hello { id: 5 };

    match my_msg {
        MyMessage::Hello { id: id_var @ 3..=7 } => println!("Found an id in range: {}", id_var),
        MyMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),

        MyMessage::Hello { id } => println!("Found some other id in range {}", id),
    }
}

fn match_guard_with_outer_var() {
    // let x = Some(5);
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n), // y is outer var not new in match scope
        _ => println!("Default case, x = {:?}", x),
    }

    let xx = 4;
    let yy = false;

    match xx {
        4 | 5 | 6 if yy => {
            println!("matched yes");
        }
        _ => {
            println!("matched no");
        }
    }
}

fn foo(_: i32, y: i32) {
    println!("y parameter is {}, and x is arbitrary", y);
}

// destruction nested enums
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn enum_test() {
    // let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    // let msg = Message::Quit;
    // let msg = Message::Move(10, 20);
    // let msg = Message::Write("hello rust");
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the {} x direction and {} y direction", x, y),
        Message::Write(text) => println!("Text message {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the rgb color to red {}, green {}, blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the hsv color to h {}, s {}, v {}", h, s, v)
        }
    }
}

fn main() {
    if_let_pattern_test();
    while_let_pattern_test();
    for_loop_pattern_test();
    let_destruction_pattern_test();
    pattern_syntax_test();
}
