// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {

let number = 101;
let is_over_hundred = number > 100;
print_statement(is_over_hundred)

}

fn print_statement(statement: bool) {
    match statement {
        true => println!("It is over 100"),
        false => println!("It is under 100")
    }
}