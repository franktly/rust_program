// struct def
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct RefUser {
    // username: &str, // missing lifetime specifier
    // email: &str,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // method function
    fn area(&self) -> i32 {
        self.width * self.height
    }

    // method function
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function(static in cpp) NO SELF
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//multiple impl struct (the same inside)
impl Rectangle {
    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}
fn main() {
    // construct from def
    let mut user1 = User {
        email: String::from("tly@163.com"),
        username: String::from("tly"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1 is {:#?}", user1);

    // modify
    user1.email = String::from("tly@gmail.com");
    println!("user1 is {:#?}", user1);

    // buid from function
    let user2 = build_user(String::from("frank"), String::from("frank@163.com"));
    println!("user2 is {:#?}", user2);

    // buid from function (simple)
    let user3 = build_user_simple(String::from("franksim"), String::from("franksim@163.com"));
    println!("user3 is {:#?}", user3);

    // create instance from other instance with struct update syntax
    let user4 = User {
        email: String::from("tly@163.com"),
        username: String::from("tly"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };
    println!("user4 is {:#?}", user4);

    // create instance from other instance with struct update syntax (simple) base struct
    // inheriated from  user1
    let user5 = User {
        email: String::from("tly@163.com"),
        username: String::from("tly"),
        ..user1
    };
    println!("user5 is {:#?}", user5);

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black = {},{},{}", black.0, black.1, black.2);
    println!("origin = {},{},{}", origin.0, origin.1, origin.2);

    let user_ref: RefUser = RefUser {
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };
    println!("user_ref is {:#?}", user_ref);

    // normal
    let w1 = 20;
    let h1 = 40;
    println!("The area of rectangle is {} square pixels", area(w1, h1));

    // tuple
    let rect1 = (w1, h1);
    println!(
        "The area of rectangle is {} square pixels",
        area_tuple(rect1)
    );

    //struct
    let rect2 = Rectangle {
        width: w1,
        height: h1,
    };

    println!("rect2 is ({}, {})", rect2.width, rect2.height);
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    let rect3 = Rectangle {
        width: 10,
        height: 10,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 100,
    };

    // struct impl method
    println!("rect2 area is {}", rect2.area());
    println!("can rect2 hold rect3 : {}", rect2.can_hold(&rect3));
    println!("can rect2 hold rect4 : {}", rect2.can_hold(&rect4));

    let rect5 = Rectangle::square(30);
    println!("rect5 perimeter is {}", rect5.perimeter());
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_tuple(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_simple(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
