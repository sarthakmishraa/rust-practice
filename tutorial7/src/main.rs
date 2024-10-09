use std::io;

fn main() {
    // let condition = 2<33;
    // println!("{}", condition);

    // let condition = (2 as f32) < 33.2;
    // println!("{}", condition);

    // let condition = (2 as f32) < 33.2;
    // let condition2 = false && condition;
    // let condition3 = false || condition;
    // println!("Using && (and operator) : {}", condition2);
    // println!("Using || (or operator) : {}", condition3);

    // let condition4 = true || false;
    // let condition5 = !(condition4);
    // println!("Using not(!) operator : {}", condition5);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");
    let input_string: i64 = input.trim().parse().unwrap();

    if input_string%2==0 && input_string<200{
        println!("You entered {} which is even", input_string);
    }
    else if input_string%2==0 && input_string>200{
        println!("You entered {} which is odd, even though its greater than 200", input_string);
    }
    else if input_string%2!=0 && input_string>200{
        println!("You entered {} which is odd, even though its greater than 200", input_string);
    }
    else {
        println!("You entered {} which is odd", input_string);
    }
}

// tutorial 7 Conditions and Control Flow (if/else if/else)

// and => &&
// or => ||
// not => !

// if not using parentheses, apply 'not' operator first then 'and' and then 'or'