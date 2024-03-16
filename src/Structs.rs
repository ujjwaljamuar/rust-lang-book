use std::u32;

struct User {
    username: String,
    sex: char,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        sex: 'M',
        username: String::from("ujjwaljamuar"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "username - {} \nsex - {} \nactive - {} \nsign-in-count - {} \n\n",
        user1.username, user1.sex, user1.active, user1.sign_in_count
    );

    user1.active = false;

    println!(
        "username - {} \nsex - {} \nactive - {} \nsign-in-count - {} \n\n",
        user1.username, user1.sex, user1.active, user1.sign_in_count
    );

    let user2 = build_user(String::from("arshdeepdubey"), 'M');

    println!(
        "username - {} \nsex - {} \nactive - {} \nsign-in-count - {} \n\n",
        user2.username, user2.sex, user2.active, user2.sign_in_count
    );

    let user3 = User {
        username: String::from("someshkumar"),
        sex: 'M',
        ..user2
    };

    println!(
        "username - {} \nsex - {} \nactive - {} \nsign-in-count - {} \n\n",
        user3.username, user3.sex, user3.active, user3.sign_in_count
    );

    let _color1 = Color(121, 121, 121);

    // let rect = (30, 50);
    // println!("The area of rectangle = {}", area_of_rectangle(rect));

    let rect = Rectangle {
        length: 50,
        width: 40,
    };

    let rect1 = Rectangle {
        length: 30,
        width: 30,
    };

    let rect2 = Rectangle {
        length: 60,
        width: 60,
    };

    println!("dimensions of {:#?}", rect);

    println!(
        "The area of rectangle = {} unit square.",
        area_of_rectangle(&rect)
    );
    println!("\nSame thing using impl");
    println!(
        "The area of rectangle = {} unit square.",
        rect.area_of_rectangle()
    );

    println!("Can hold rect1 inside - {} ", rect.can_hold_inside(&rect1));
    println!("Can hold rect1 inside - {} ", rect.can_hold_inside(&rect2));

    let rect3 = Rectangle::square(25);
    println!("{:#?}", rect3);
}

fn area_of_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area_of_rectangle(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold_inside(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
// fn area_of_rectangle(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

struct Color(i32, i32, i32);

fn build_user(username: String, sex: char) -> User {
    User {
        username,
        sex,
        active: true,
        sign_in_count: 1,
    }
}
