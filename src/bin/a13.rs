// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector



fn main() {
    let my_numbers = vec![1, 2, 3, 4];

    for num in &my_numbers  {
        match num {
            3 => println!("three"),
            _ => println!("{:?}", num)
    }
}

println!("Number of elements in vector: {:?}", my_numbers.len());
}