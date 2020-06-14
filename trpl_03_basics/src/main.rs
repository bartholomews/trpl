fn variables() {
    // `let`
    // => are immutable by default
    // => add `mut` to create a variable
    let mut x = 5; // default is `i32`
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);
    // `const`
    // => MUST annotate type
    // => MUST be set to a constant expression, not a function call
    // naming convention is SCREAMING_SNAKE_CASE (also for numeric)
    const MAX_POINTS: u32 = 100_000;
    // shadowing values
    let x: u32 = MAX_POINTS;
    println!("The value of x is {}", x);
    let x: i32 = 10;
    println!("The value of x is {}", x);
}

fn data_types() {
    // unsigned: 0 to (2en)-1
    // signed: -(2en-1) to (2en-1) - 1
    let _: u8 = 1; // 0 to 255
    let _: i8 = 1; // -128 to 127
    let _: u16 = 1; // 0 to 65,535
    let _: i16 = 1; // -32,768 to 32,767
    let _: u32 = 1; // 0 to ~ 4 billion
    let _: i32 = 1; // ~ -2 billion to ~ 2 billion
    let _: u64 = 1; // ~ 18.4 quintillion
    let _: i64 = 1; // ~ -9.2 quintillion to ~ 9.2 quintillion
    let _: u128 = 1; // ~ 340 duodecillion
    let _: i128 = 1; // ~ -170 duodecillion to 170 duodecillion
    let _: isize = 1; // depends on architecture: x64 or x32
    let _: usize = 1; // depends on architecture: x64 or x32
    // integer literals
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _binary = b'A'; // u8 only
    // floating point numbers
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    // bool (1B)
    let _a = true;
    // char (4B) Unicode Scalar Value [U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive]
    let _heart_eyed_cat = '😻';
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring tuple
    let (_x, _y, _z) = tup;
    // accessing tuple elements
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    // array (fixed length allocated on the stack)
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // syntactic sugar for [3, 3, 3, 3, 3]
    let _a = [3; 5];
    // access indexes
    let _first = _a[0];
    let _second = _a[1];
}

fn main() {
    variables();
    data_types();
}