use std::{collections::HashMap, hash::Hash};

fn main() {
    let _a = [1, 3, 5];

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    // print the first element
    println!("{}", &v[0]);

    for element in &v {
        print!("{} ", element)
    }
    println!("");

    let mut v2 = vec![1, 3, 4];
    v2.push(5);

    for element in &v2 {
        print!("{} ", element);
    }
    println!();

    match &v2.get(2) {
        Some(third) => println!("{} is the third element", third),
        None => println!("No third element"),
    }

    // we can modfiy element while using it
    for element in &mut v2 {
        *element += 20;
        print!("{} ", element);
    }
    println!();

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("Ujjwal Jamuar")),
        SpreadsheetCell::Float(1.121),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}, which is an integer", i),
        _ => println!("Not an integer"),
    };

    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores{
        println!("{} - {}", key, value);
    }

    // override the value
    scores.insert(String::from("blue"), 20);

    for (key, value) in &scores{
        println!("{} - {}", key, value);
    }

    // if key exist do nothing 
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(60); // do nothing

    println!();
    for (key, value) in &scores{
        println!("{} - {}", key, value);
    }

    let mut map:HashMap<&str, i32> = HashMap::new();

    let text = "Hello world dear world";

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // {"world": 2, "Hello": 1, "dear": 1}
}
