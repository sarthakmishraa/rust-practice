use std::io;

fn main() {
    // println!("Hello, world!");
    // println!("This is tutorial 6.");

    // let x: u8 = 9; // 0 to 255
    // let y: i8 = 10; // -128 to 128

    // println!("x is {} \ny is {}", x, y);

    // let x: f32 = 9.0;
    // let y: f64 = 10.0;

    // let x: f32 = 9.0;
    // let y: f32 = 10.0;
    // let z = x + y;
    // println!("sum is {}", z);

    // let x: f32 = 9.0;
    // let y: f32 = 10.0;
    // let z = x - y;
    // println!("difference is {}", z);

    // let x2: f32 = 90.0;
    // let y2: f32 = 10.0;
    // let z2 = x2 / y2;
    // println!("Result of division is {}", z2);

    // let x2: f64 = 95.0;
    // let y2: f64 = 10.0;
    // let z2 = x2 / y2;
    // println!("Result of division when f64 used is {}", z2);

    // let x3: f32 = 90.0;
    // let y3: f32 = 10.0;
    // let z3 = x3 * y3;
    // println!("Result of multiplication is {}", z3);

    // let x3: f32 = 99.0;
    // let y3: f32 = 10.0;
    // let z3 = x3 % y3;
    // println!("Result of mod is {}", z3);

    // let x3 = 99i8;
    // let y3 = 10i8;
    // let z3 = x3 % y3;
    // println!("Result of mod is {}", z3);

    // let x3 = 99_00i64;
    // let y3 = 10 as i64;
    // let z3 = x3 / y3;
    // println!("Result of mod is {}", z3);

    // let a = 5 as i8;
    // let b = 6 as i64;
    // let c = (a as i64) + b;
    // println!("Result of sum after typecasting is {}", c);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("expected to read line");
    let input_string: i64 = input.trim().parse().unwrap();

    println!("You entered : {}", input_string);
    println!("If we add 2 to it that becomes : {}", input_string+2);
}

// tutorial 6 - arithematic and type casting

// cannot use arithematic operators on different dataypes

// ways to define variables with a specific datatypes
// let x: i8 = 5
// let x = 5_i8
// let x = 5i8
// let x = 5 as i8

// typecasting
// let a = 5 as i8
// let b = 6 as i64
// let c = (a as i64) + b
// parenthesis are not required for typecasting

// string to int
// let input_string: i64 = input.trim().parse().unwrap();
// trim() gets the string that was input from command line interface
// parse() return the result which parse the string in to datatype assigned to the variable (i64 in this case)
// unwrap() take any valid integer result and unwrap it in to actual integer type