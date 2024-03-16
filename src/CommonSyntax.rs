fn main() {

    // mutable 

    let mut x = 3;
    println!("x = {}",x);
    x= 2;
    println!("x = {}",x);

    // shadowing
    let y = 4;
    println!("y = {}",y);
    let y = "four";
    println!("y = {}",y);

    /*
    shadowing can mutate variables , we can redeclare it using same name and can change data types also

    */


    // constants - cannot be changed even with mut

    const SUBSCRIBER_COUNT : i32 = -1_00_000;    // we can made it readable using underscore

    println!("Subscribers  = {}",SUBSCRIBER_COUNT );
}
