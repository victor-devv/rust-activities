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

enum Flavour {
    Orange,
    Apple,
    Peach,
    Blackberry,
}

struct Drink {
    flavour: Flavour,
    ounce: u32, //u - unsigned integer, i - signed integer, f - floating point
}

fn print_drink(drink: Drink) {
    match drink.flavour {
        Flavour::Orange => println!("flavour: orange"),
        Flavour::Apple => println!("flavour: apple"),
        Flavour::Blackberry => println!("flavour: blackberry"),
        Flavour::Peach => println!("flavour: peach"),
    };

    println!("oz: {:?}", drink.ounce);
}

fn main() {
    let drink: Drink = Drink {
        flavour: Flavour::Peach,
        ounce: 16
    };

    print_drink(drink);
}
