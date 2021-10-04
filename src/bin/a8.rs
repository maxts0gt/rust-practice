// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


enum DrinkType {
    Espresso,
    Americano,
    Latte,

}


struct Drink {
    flavor: DrinkType,
    fluid_ml: f64,
}


fn print_drink(drink: Drink) {
    match drink.flavor {
        DrinkType::Espresso => println!("Espresso"),
        DrinkType::Americano => println!("Americano"),
        DrinkType::Latte => println!("Latte"),
    }
    println!("oz: {:?}", drink.fluid_ml)
}

fn main() {
    let morning_drink = Drink {
        flavor: DrinkType::Espresso,
        fluid_ml: 50.0
    };
    print_drink(morning_drink);

    let afternoon_drink = Drink {
    flavor: DrinkType::Latte,
    fluid_ml: 200.0
    };
    print_drink(afternoon_drink)
}
