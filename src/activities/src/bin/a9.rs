// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_coordinates() -> (f64, f64) {
    (-3.0, 1.0)
}

fn main() {
    let (_x, y) = get_coordinates(); //_x so the compiler ignores the fact that it isn't used

    if y > 5.0 {
        println!("y is greater than 5");
    } else if y < 5.0 {
        println!("y is less than 5");
    } else if y == 5.0 {
        println!("y is equal to 5");
    }

}
