fn main() {
    println!("Enter a color: ");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = String::from(input.trim());

    if input == "red" {
        println!("The color is red!");
    } else if input == "blue" {
        println!("The color is blue!");
    } else {
        println!("The color is not correct!");
    }
}
