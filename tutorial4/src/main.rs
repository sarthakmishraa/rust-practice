fn main() {
    // -------------------------------------scalar types

    // let x : i32 = 2;
    // let y : u32 = 250;

    // let float_point: f32 = 10.9;

    // let t_or_f1: bool = false;

    // let t_or_f3: bool = true;

    // println!("{}", t_or_f1);
    // println!("{}", t_or_f3);

    // let letter: char = 'a';

    // println!("{}", letter);

    // -------------------------------------compound types

    // tuples
    // let tups_1: (i32, i32, i32, i32, i32, char, bool) = (1, 2, 3, 4, 5, 'a', true);
    // println!("{}", tups_1.6);

    // let mut tups: (i32, i32, i32, i32, i32, char, bool) = (1, 2, 3, 4, 5, 'a', true);
    // println!("{}", tups.5);
    // tups.5 = 'x';
    // println!("{}", tups.5);

    // let mut tups: (i32, i32, i32, i32, i32, char, bool) = (1, 2, 3, 4, 5, 'a', true);
    // println!("{}", tups.1);
    // tups = (55, 4, 8, 9, 25, 'p', false);
    // println!("{}", tups.1);

    // arrays
    // let mut arr1 = [0, 1, 2, 3, 4, 5, 6];
    // println!("{}", arr1[2]);
    // arr1[2] = 20;
    // println!("{}", arr1[2]);

    // let arr2: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    // println!("{}", arr2[2]);

    // let arr3: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    // println!("{}", arr3[2]);

    // let x: u8 = 4;
    // let y = x;
    // both x and y has the same datatype (u8 in this case)
    println!("{}, {}", x, y);

    let x: u8 = 4;
    let y: i32 = x;
    // this gives an error that datatype of x was expected to be i32 but found u8, so we need need to convert that
    println!("{}, {}", x, y);
}

// tutorial 4 - datatypes

// -------------------------------------scalar types
// by default integers have i32 datatype
// i32 = signed integer 32
// i8, i16, i64, i128 (more signed integer datatypes)

// unsigned integer cannot have negative values
// u32 = unsigned integer 32
// u8, u16, u64, u128

// f32(default), f64

// -------------------------------------compound types

// tuples
// they are immutable by default, we can make them mutable by using mut keyword
// let tuple_name: (datatype1, datatype2... , datatypen) = (value1, value2... , valuen)
// datatypes should correspond to their respective values

// arrays
// they are immutable by default, we can make them mutable by using mut keyword
// they are int32 by default
// use array[i] to index array elements

// let array: [datatype_for_array; no_of_elements] = [0, 1, 2, 3... , n];
// n = no_of_elements

// value does not get initialized automatically, if we do not initialize any element in a array we get a compile time error