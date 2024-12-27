// example of a library crate with a lib.rs file - consumable
// other types of crates are binary crates with a main.rs file - runnable
mod back_of_house {
    // marking enum as public allows us to use its variants
    // enum variants are public by default while struct fields/methods are private by default
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
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
}

use back_of_house::Appetizer;
use back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    let soup = Appetizer::Soup;
    let salad = Appetizer::Salad;
    println!("{:#?}{:#?}", soup, salad);
    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}