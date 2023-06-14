fn main() {
    // You cant add different types, for example: i32 + i64
    let x: f64 = 255.0;
    let y: f64 = 2.0;

    let z = x / y;
    println!("z = {}", z);

    // powf for floats
    let a = x.powf(y);
    // checked_powf guards against overflow
    println!("a = {}", a);

    // Cast values
    let x: f32 = 255.0;
    let y: f64 = 2.0;

    let z = (x as f64) / y; // Parenthesis are not needed
    println!("z = {}", z);

    // Cast user input to a number
    println!("Enter an integer number: ");
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap(); // Can use a crate without importing it first

    let input: i64 = input.trim().parse().unwrap(); // trim() removes whitespace, parse() converts to a number,
                                                    // unwrap() returns the value or panics if it fails

    println!("input + 1 = {}", input + 1);
}
