pub mod hosting {
    pub fn add_to_waitlist() {}
    pub fn seat_at_table() {}
}

pub mod serving {
    pub fn take_order() {}
    fn serve_order() {}
    pub fn take_payment() {}

    pub mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peachs"),
                }
            }
        }

        fn cook_order() {}

        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }
}
