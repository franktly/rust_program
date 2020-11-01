#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrBind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrSepBind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                    // no data associated
    Move { x: i32, y: i32 }, // anonymouse struct
    Write(String),           // string
    Change(i32, i32, i32),   //three i32
}

//  enum holds struct
struct QuitMessage; // uint struct (none)
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeMessage(i32, i32, i32); // tuple struct
                                     //  enum holds impl method
impl Message {
    fn call(&self) {
        println!("message call invoked.");
    }
}

// enum template
// enum Option<T> {
// Some(T),
// None,
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four is {:?}, six is {:?}", four, six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home is {:#?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback is {:#?}", loopback);

    let home = IpAddrBind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrBind::V6(String::from("::1"));
    println!("home bind is {:#?}", home);
    println!("loopback bind is {:#?}", loopback);

    let home = IpAddrSepBind::V4(127, 0, 0, 1);
    println!("home seperate bind is {:#?}", home);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_num = Some(5); // auto infer
    println!("some_num is {:?}", some_num);
    let some_str = Some("a string"); // auto infer
    println!("some_str is {:?}", some_str);
    let some_none: Option<i32> = None; // explicitly innota
    println!("some_none is {:?}", some_none);

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; cannot add i8 and Option<i8>

    let p = value_in_cents(Coin::Penny);
    println!("p is {}", p);
    let n = value_in_cents(Coin::Nickel);
    println!("n is {}", n);
    let d = value_in_cents(Coin::Dime);
    println!("d is {}", d);
    let q = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("q is {}", q);

    let five = Some(5);
    let six = plus_one(five);
    println!("six is {:?}", six);
    let none = plus_one(None);
    println!("none is {:?}", none);

    // let some_u8_value = 0u8;
    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // match _()
    let some_opt_u8 = Some(0u8);
    // let some_opt_u8 = Some(3u8);
    match some_opt_u8 {
        Some(3) => println!("some 3"),
        _ => (),
    }

    // if let expression
    if let Some(3) = some_opt_u8 {
        println!("some 3");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(us_state) => println!("state quarter from {:?}", us_state),
        _ => count += 1,
    }

    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(us_state) = coin {
        println!("state quarter from {:?}", us_state);
    } else {
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny coin");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => {
            println!("Quarter UsState is {:?}", us_state);
            25
        }
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("current ip kind is {:#?}", ip_kind);
}
