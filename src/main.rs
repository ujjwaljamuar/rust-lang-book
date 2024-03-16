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
}

fn build_user(username: String, sex: char) -> User {
    User {
        username,
        sex,
        active: true,
        sign_in_count: 1,
    }
}
