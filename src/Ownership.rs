fn main() {
    /*
        Ownership
        1. there can only be on owner of a variable
        2. one owner at a time
        3. when owner goes out of scope, the variable is dropped from memory
    */

    let normal_string = "Ujjwal Jamuar"; // cannot be modfied

    let heap_string = String::from("Ujjwal Jamuar"); // stores new string in heap memory

    let x = 5;
    let y = x;
    println!("{}", x);

    let a = String::from("Ujjwal");
    let b = a; // it is moved, (not shallow copy) value is borrowed and ownership is transfered to b
    println!("{}", b);

    // cloning a value without borrowing

    let c = String::from("Hello");
    let d = c.clone();
    println!("{} {}", c, d);

    let s = String::from("Hello world");
    takes_ownership(s); // owner ship moved
    // println!("{}",s);  // will cause error

    let s1 = gives_ownership();
    println!("{}", s1)


}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn gives_ownership() -> String{
    let some_string = String::from("Hello world!!!");

    some_string
}