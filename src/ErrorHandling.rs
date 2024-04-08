use core::panic;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // panic!("crash and burn");

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        // Err(error) => panic!("Problem opening this file {:?}", error),
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match  File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}",e),
            }
            other_error => {
                panic!("Problem opening this file: {:?}", other_error);
            }
        },
    };

    let f = File::open("hello.txt").expect("failed to open this file");

    

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    
}

fn read_something() -> Result<String, io::Error>{
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}