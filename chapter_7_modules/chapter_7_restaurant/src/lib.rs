mod front_of_house;
mod back_of_house;
/* bring the hosting module into scope */
use crate::front_of_house::{ hosting, serving };
use crate::back_of_house::{ Breakfast, Appetizer };


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    serving::take_order();

    /* order breaskfast */
    let mut meal = Breakfast::summer("Rye");

    /* change order */
    meal.toast = String::from("Wheat");
    println!("Ordered{}", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

