fn main() {
    const MAX_POINT: u32 = 100_000;

    // mut variable
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // const variable
    println!("max point is: {}", MAX_POINT);

    // shadowing
    let y = 5;
    println!("The origin y is {}", y);
    let y = y + 1;
    println!("The added by one is {}", y);
    let y = y * 2;
    println!("The multiplied by two is {}", y);

    // data type
    let x: i8 = 10;
    println!(" i8 x is {}", x);

    let x: u8 = 10;
    println!(" u8 x is {}", x);

    let x: i16 = 10;
    println!(" i16 x is {}", x);

    let x: u16 = 10;
    println!(" u16 x is {}", x);

    let x: i32 = 10;
    println!(" i32 x is {}", x);

    let x: u32 = 10;
    println!(" u32 x is {}", x);

    let x: i64 = 10;
    println!(" i64 x is {}", x);

    let x: u64 = 10;
    println!(" u64 x is {}", x);

    let x: i128 = 10;
    println!(" i128 x is {}", x);

    let x: u128 = 10;
    println!(" u128 x is {}", x);

    let x: isize = 10;
    println!(" isize x is {}", x);

    let x: usize = 10;
    println!(" usize x is {}", x);

    let x: u32 = 88_333; // Decimal
    println!(" decimal x is {}", x);

    let x: u32 = 0xff; // Hex
    println!(" decimal x is {}", x);

    let x: u32 = 0o77; // Octal
    println!(" octal x is {}", x);

    let x: u32 = 0b1111_0000; // Binary
    println!(" binary x is {}", x);

    let x: u8 = b'A'; // Byte
    println!(" byte x is {}", x);

    let x = 2.0; // f64
    println!(" f64 x is {}", x);

    let x: f32 = 3.0; // f32
    println!(" f32 x is {}", x);

    // numeric operations
    let sum = 5 + 10;
    println!("sum of 5 + 10 is {}", sum);

    let diff = 95.5 - 4.3;
    println!("diff of 95.5 - 4.3 is {}", diff);

    let product = 4 * 30;
    println!("product of 4 * 30 is {}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient of 56.7/32.2 is {}", quotient);

    let remainder = 43 % 5;
    println!("remainder of 43%5 is {}", remainder);

    let t = true;
    println!("t is {}", t);

    let f: bool = false;
    println!("f is {}", f);

    // the character type
    let c = 'z';
    println!("c is {}", c);

    let z = 'Z';
    println!("z is {}", z);
    // let heart_eyed_cat = U + E000;
    //
    // compound types

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("tup is ({}, {},{})", tup.0, tup.1, tup.2);
    println!("tup to abc is ({}, {},{})", a, b, c);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    // let invalid = arr[10];
    println!("first is {}", first);

    for (i, item) in arr.iter().enumerate() {
        println!("arr[{}] is {}", i, item);
    }

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    for i in months.iter() {
        println!("month is {}", i);
    }

    let aaa: [i32; 5] = [2, 3, 4, 5, 6]; // number is 5 with data type is i32

    for i in aaa.iter() {
        println!("aaa is {}", i);
    }

    let bbb = [3; 5]; // number is 5 with initialized value 3

    for i in bbb.iter() {
        println!("bbb is {}", i);
    }

    another_function();
    another_function_p1(88);
    another_function_p2(6, 6);

    statements_and_expressions();

    let x = fun_with_returns(5);
    println!("The value 5 + 1 is {}", x);

    let number = 3;
    if number < 5 {
        println!("little than 5 condition was true");
    } else {
        println!("bigger than 5 condition was false");
    }

    if number != 0 {
        println!("number was no zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3,2");
    }

    let condition = true;
    let res = if condition { 5 } else { 6 };
    println!("res is {}", res);

    // loop {
    // println!("again!");
    // }

    let mut cnt = 0;

    let result = loop {
        cnt += 1;

        if cnt == 10 {
            break cnt + 2;
        }
    };

    println!("loop return {}", result);

    let mut cnt = 3;
    while cnt != 0 {
        println!("{}!", cnt);
        cnt -= 1;
    }

    for num in (1..4).rev() {
        println!("rev of (1..4) is {}", num);
    }
}

fn another_function() {
    println!("Another function");
}

fn another_function_p1(x: i32) {
    println!("Another function with param {}", x);
}

fn another_function_p2(x: i32, y: i32) {
    println!("Another function with p1: {}, p2: {}", x, y);
}

fn statements_and_expressions() {
    let x = 5;
    println!("The value of x is: {}", x);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn fun_with_returns(x: i32) -> i32 {
    x + 1
}
