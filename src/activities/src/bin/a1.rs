// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// * Use a function to display your first name
fn first_name() {
    let first_name = "Victor";
    println!("{first_name}");
}

// * Use a function to display your last name
fn last_name() {
    let last_name = "Ayanfeogo";
    println!("{last_name}");
}

fn main() {
    first_name();
    last_name();
}
//cargo run --bin a1
//cargo run -q --bin a1
