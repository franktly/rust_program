mod front_of_house; // separate modules into different files, sub module name should be front_of_house.rs!!!( no need to add pub mod declaring in front_of_house.rs module file)
                    // rel
                    // use self::front_of_house::serving::back_of_house;
                    // abs
                    // use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting; // pub use
pub use crate::front_of_house::serving; // pub use
pub fn eat_at_restaurant() {
    // abs path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    // rel
    // let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    let mut meal = serving::back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    let order1 = serving::back_of_house::Appetizer::Soup;
    let order2 = serving::back_of_house::Appetizer::Salad;
}
