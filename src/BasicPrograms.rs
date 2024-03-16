fn main(){
    sum(1, 2);
    println!("{}", sum1(5, 6));

    // if else
    let num = 1;

    // condition must be a boolean value
    if num < 6 {
        println!("number is less");
    } else if num > 6 {
        println!("number is greater");
    } else {
        println!("number is equal");
    }

    // control flow
    let condition = true;
    let n = if condition { 6 } else { 10 };
    println!("{}", n);

    // loops
    let mut number = 10;
    loop {
        print!("{} ", number);
        number += 1;

        if number == 15 {
            break;
        }
    }

    println!("\nreturn loop");

    let mut number2 = 10;

    // return a number ffrom loop

    let result = loop {
        print!("{} ", number2);
        number2 += 1;

        if number2 == 15 {
            break number2;
        }
    };
    println!("\nReturned value -> {}", result);

    println!("while loop");
    let mut a = 0;
    while a < 10 {
        print!("{} ", a);
        a += 2;
    }
    print!("\n");

    // for loop
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("{}", element);
    }

    // range
    for element in 1..5 {
        println!("{}", element);
    }
}

// not returning anything
fn sum(value1: i32, value2: i32) {
    println!("{}", value1 + value2);
}

// return something
fn sum1(value1: i32, value2: i32) -> i32 {
    // return value1 + value2;

    // do without return keyword just write without semicolon means the value is getting returned
    value1 + value2
}
