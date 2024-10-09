fn main() {
    // println!("This is the main function");
    test();
    add_two_numbers(4,7);

    // let number = {
    //     let x = 5;
    //     x + 1
    // };
    // println!("number is : {}", number);

    // let number = {
    //     let mut x = 5;
    //     x = x+1;
    //     x+4
    // };
    // println!("number is : {}", number);

    let num1 = 4;
    let num2 = 5;
    let product = multiply_two_numbers(num1, num2);
    println!("Product of {} and {} : {}", num1, num2, product);

    println!("{}", return_early(2, 5));
}

fn test(){
    println!("This is test function");
}

fn add_two_numbers(x: i32, y: i32){
    println!("Sum of {} and {} : {}", x, y, x + y);
}

fn multiply_two_numbers(x: i32, y: i32) -> i32{
    x*y
    // return x*y;
}

fn return_early(x: i32, y: i32) -> i32{
    let res = x+y * (x*y);
    println!("Value of res : {}", res);
    if res<80{
        return res-20;
    }
    res
}

// tutorial 8 Functions, Expressions & Statements

// to return something from a function, make sure to keep an expression at the last statement of the function body
// do not add semicolon at the last of expression in a function or else it will not return
// can also use 'return' keyword to return, can use semicolon when 'return' keyword used

// if returning early, use 'return' keyword in a function, see example return_early() method

// if we do not define datatype of returning value, we get error