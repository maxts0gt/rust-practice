// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_person(name: &str, age: i32, color: &str) {
    println!("name: {:?}", name);
    println!("age: {:?}", age);
    println!("color: {:?}", color)
}



fn main() {
    let people = vec![
        Person {
            age: 10,
            name: "John".to_owned(),
            color: String::from("red")
        }, 
        Person {
            age: 5,
            name: "Jane".to_owned(),
            color: String::from("green")
        }, 
        Person {    
            age: 12,
            name: "Jason".to_owned(),
            color: String::from("blue")
        }, 
    ];

    for person in people {
        match person.age {
        10 => println!("John came! He is 10. We shouldn't print him"),
        _ => print_person(&person.name, person.age, &person.color)
    }
}

}
