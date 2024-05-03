// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: i32,
    quantity: i32,
}

fn display_id(grocery: &Grocery) {
    println!("id is: {:?}", grocery.id);
}

fn display_quantity(grocery: &Grocery) {
    println!("quantity is {:?}", grocery.quantity);
}

fn main() {
    let corn_flakes: Grocery = Grocery {
        id: 25,
        quantity: 200,
    };

    display_id(&corn_flakes);
    display_quantity(&corn_flakes);
}
