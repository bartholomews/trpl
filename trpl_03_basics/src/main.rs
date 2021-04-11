mod temperature_converter;
mod nth_fibonacci;
mod the_twelve_days_of_christmas;

fn main() {
    for i in 0..10 {
        line_separator();
        println!("{} -> {}", i, nth_fibonacci::fib(i));
        println!("{} -> {}", i, nth_fibonacci::fib_rec(i));
    }
    line_separator();
    println!("{}°C = {}°F", 232.77779, temperature_converter::to_fahrenheit(232.77779));
    println!("{}°F = {}°C", 451, temperature_converter::to_celsius(451.00));
    line_separator();
    the_twelve_days_of_christmas::print_lyrics();
}

#[allow(dead_code)]
fn line_separator() {
    println!("{}", str::repeat("~", 100));
}

#[allow(dead_code)]
fn variables() {
    // `let`
    line_separator();
    println!("`let`");
    let mut x = 5; // default is `i32`
    println!("- are immutable by default`");
    println!("- add `mut` to create a variable\n");
    println!("[let mut x = 5] The value of x is {}", x);
    x = 10;
    println!("[x = 10] The value of x is {}", x);
    // `const`
    line_separator();
    println!("`const`");
    println!("- MUST annotate type");
    println!("- MUST be set to a constant expression, not a function call");
    println!("- naming convention is SCREAMING_SNAKE_CASE (also for numeric)");
    const MAX_POINTS: u32 = 100_000;
    // shadowing values
    line_separator();
    println!("shadowing values");
    let x: u32 = MAX_POINTS;
    println!("[let x: u32 = MAX_POINTS] The value of x is {}", x);
    let x: i32 = 10;
    println!("[let x: i32 = 10] The value of x is {}", x);
}

#[allow(dead_code)]
fn data_types() {
    line_separator();
    println!("scalar types = [Integer | Floating-Point | Boolean | Char]");
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // integers:
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
    // number literals
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _binary = b'A'; // u8 only
    /*
    integer overflow:
    `panic!` (if compiled without `--release` flag)
    `two’s complement wrapping` (if compiled with `--release` flag)
    overflow handling on primitive numeric types:
    - `wrapping_*` (wrap in all modes - e.g. `wrapping_add`)
    - `checked_*` (return the `None` value if there is overflow)
    - `overflowing_*` (return the value and a boolean indicating whether there was overflow)
    - `saturating_*` (saturate at the value's minimum or maximum values)
     */
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Floating-Point
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Boolean (1B)
    let _a = true;
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Char (4B) Unicode Scalar Value [U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive]
    let _heart_eyed_cat = '😻';
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Compound types
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring tuple
    let (_x, _y, _z) = tuple;
    // accessing tuple elements
    let _five_hundred = tuple.0;
    let _six_point_four = tuple.1;
    let _one = tuple.2;
    // array (fixed length allocated on the stack)
    let _array: [i32; 5] = [1, 2, 3, 4, 5];
    let _array = [3; 5]; // syntactic sugar for [3, 3, 3, 3, 3]
    // access indexes
    let _first = _array[0];
    let _second = _array[1];
}

#[allow(dead_code)]
#[allow(path_statements)]
fn functions() {
    // functions
    line_separator();
    // In function signatures, you *must* declare the type of each parameter
    fn another_function(x: i32, y: i32) -> () {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
        // Function bodies contain statements and expressions
        // statement
        let y = {
            let x = 3; // statement
            println!("{}", x + y); // statement
            x + y // expression
        };
        y; // statement (function return type is `()` instead of `i32`)
    }

    another_function(1, -1)
}

#[allow(dead_code)]
fn control_flow() {
    // `if` expressions
    line_separator();
    println!("`if` expressions");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //
    println!();
    println!("we can use an `if` expression on the right side of a let statement");
    println!("all branches need to have compatible types");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("let number = if condition {{ 5 }} else {{ 6 }};");
    println!("{}", number);
    // loop
    line_separator();
    println!("`loop`");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // statement that assign the value to `result`
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    // `while loop`
    line_separator();
    println!("`while loop`");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    // `for loop`
    line_separator();
    println!("`for loop`");
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // countdown with `for loop`
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}