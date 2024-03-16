fn main() {
    // References

    let s1 = String::from("Ujjwal Jamuar");
    let length = calculate_length(&s1);
    println!("{}", length);

    let mut a = 11;
    let mut b = 12;

    a = a ^ b;
    b = a ^ b;
    a = a ^ b;

    println!("{} {}", a, b);

    let mut s = String::from("My value");

    let s2 = &mut s;

    println!("{}", s2);

    let string1 = "Ujjwal Jamuar";
    // let first_name = &string1[0..7];   does the same thing
    // let last_name = &string1[7..13];
    let first_name = &string1[..7];
    let last_name = &string1[7..];
    let full_name = &string1[..];
    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", full_name);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
