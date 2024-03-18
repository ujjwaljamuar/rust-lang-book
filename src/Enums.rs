fn main(){
    // define enum Direction
    #[derive(Debug)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    // initialize and access enum variants
    let north = Direction::North;
    let east = Direction::East;
    let south = Direction::South;
    let west = Direction::West;

    // print enum values
    println!("{:?}", north);
    println!("{:?}", east);
    println!("{:?}", south);
    println!("{:?}", west);
}