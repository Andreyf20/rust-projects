// Crate name of the packages and libraries
// Adding the input and output module, or crate
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your name: ");
    // Create mutable reference to the variable input to be able to write to it
    // By default you pass a copy of a value to functions
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You typed: {}", input);
}
