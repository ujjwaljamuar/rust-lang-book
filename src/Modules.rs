// structs inside element are not public if struct is even public
// enums inside element are public if enum is public

mod back_of_house {

    pub struct Breakfast {
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // fn fix_incorrect_order(){
    //     cook_order();

    //     // access parents element
    //     super::serve_order();
    // }

    // fn cook_order(){}
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Bread");

    meal.toast = String::from("Bye");

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative pat
    front_of_house::hosting::add_to_waitlist();
}

// another example
fn serve_order() {}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
