fn main() {
    // let x: u32 = 4;
    let mut x = 4;
    println!("x is {}", x);
    x = x*2;
    println!("x times 2 is {}", x);

    // Multiple declaration of same variable
    let a = 11;
    println!("a is {}", a);
    let a = 13;
    println!("a is {}", a);
    let a = a + 13;
    println!("a is {}", a);


    // Name shadowing
    let b = 100;
    println!("b is {}", b);
    {
        // let b = 50;
        // println!("b is {}", b);
        // let b = b + 50;
        // println!("b is {}", b);
        let b = "Interior scope defined in b variable";
        println!("b is {}", b);
    }
    let b = b + 100;
    println!("b is {}", b);

    // constants in rust
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("constant value is {}", SECONDS_IN_MINUTE);
}
// we can define datatypes of variables explicitly
// or
// compiler determines datatypes of varibales implicitly

// variables in rust are immutable(cannot store multiple values)
// we need to define them with keyword 'mut' if we want to make them mutable

// we can define the same variable multiple times in rust

// If we declare a variable inside a scope, it is defined in that scope only
// we can use the defined variable from the exterior scope in the interior scope

// when defining constants, always use uppercase letters and underscores, give the type of the constant and assign a value to it
// we cannot define the same constants multiple times