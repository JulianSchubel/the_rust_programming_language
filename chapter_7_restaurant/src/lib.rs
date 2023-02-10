mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
        }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

/* bring the hosting module into scope */
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    /* Absolute path */
    crate::front_of_house::hosting::add_to_waitlist();

    /* Relative path */
    front_of_house::hosting::add_to_waitlist();

    /* call to waitlist thanks to the "use" keyword */
    hosting::add_to_waitlist();

    /* order breaskfast */
    let mut meal = back_of_house::Breakfast::summer("Rye");

    /* change order */
    meal.toast = String::from("Wheat");
    println!("Ordered{}", meal.toast);

    /* The below line will cause the program to fail to compile; do not have access to the
     * seasonal_fruit field*/
    // meal.seasonal_fruit = String::from("blueberried");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

/* super keyword example */
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast : String,
        seasonal_fruit : String,
    }
    
    impl Breakfast {
        pub fn summer(toast : &str) -> Breakfast {
            /* We need this constructor as the structure has a private field */
            Breakfast {
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

}
