fn main() {
    /*
        Data Types
        1. Scalar types: represents only a single values
        2. compound types: represent a group of values
    */

    /*
        Scalar

        1. Integers - 8, 16, 32, 64, 128, arch(architecture) bit integers, signed and unsigned
        2. Floating point numbers
        3. Boolean
        4. Character
    */

    // Integers
    let new_int = 2; // default is i32
    let a: i8 = 127; // decimal
    let b: i32 = 0xff; // hex
    let c: i32 = 0o77; // octal
    let d: i32 = 0b1111; //binary
    let e: u8 = b'A'; // byte u8 only

    println!("{}", new_int);
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);

    // Floating point numbers
    let f = 5.5; // default is f64
    let g: f32 = 55.55;
    let sum = 5 as f32 + 5.5;
    let product = 5.5 * 5.5;
    let quotient = 5.5 / 5.5;
    let remainder = 5.5 % 5.5;

    println!("{}", f);
    println!("{}", g);
    println!("{}", sum);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);

    // Boolean

    let t = true;
    let fa: bool = false;

    println!("{}", t);
    println!("{}", fa);

    // Characters

    let char_a = 'A';
    let char_b: char = 'B';

    println!("{}", char_a);
    println!("{}", char_b);

    // Compound Type

    // Tuples

    let new_tuple = ("ujjwal jamuar", 12);

    // use tuples value - destructure or indexing
    let (name, age) = new_tuple;
    println!("{}", name);
    println!("{}", age);

    let name_1 = new_tuple.0;
    let age_1 = new_tuple.1;
    println!("{}", name_1);
    println!("{}", age_1);

    // Arrays - fixed length cannot be changedDat
    let array_numbers = [1, 2, 3, 4, 5];
    let number_1 = array_numbers[0];
    println!("{}", number_1);

    // create an array of size 8 with 0 as all elements
    let array_numbers_1 = [0; 8];
    for i in array_numbers_1 {
        print!("{} ", i);
    }
}
