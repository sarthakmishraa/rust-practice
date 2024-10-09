use std::io;

fn main() {
    let mut input_variable = String::new();

    println!("Enter your input : ");

    io::stdin().read_line(&mut input_variable).expect("failed to read line");
    
    println!("{}", input_variable);
}

// tutorial 5 - user input
// use package::module::method;

// let mut input_variable = String::new();
// input must always be empty string datatype

// read_line(&mut input_variable) - &mut makes the input referenced input, means
// the input_variable that we defined can have a different value that we type in command line interface
// expect() is to catch errors while taking input