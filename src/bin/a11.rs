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

struct Item {
    quantity: i32,
    id: i32
}


fn display_item_move(item: Item) {
    println!("Quantity: {}", item.quantity);
    println!("Quantity: {}", item.id);
}
fn display_quantity_borrow(item: &Item) {
    println!("Quantity: {}", item.quantity);

}
fn display_id_borrow(item: &Item) {
    println!("ID: {}", item.id);
}



fn main() {
    let cookie = Item {
        quantity: 5,
        id: 7
    };
    display_quantity_borrow(&cookie);
    display_id_borrow(&cookie);
    display_item_move(cookie)

}
