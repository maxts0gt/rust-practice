// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(selected_color: Color) {
        match selected_color {
            Color::Red => println!("It is Red"),
            Color::Green => println!("It is Green"),
            Color::Blue => println!("It is Blue"),
    }
}

fn main() {
    print_color(Color::Green);
}
