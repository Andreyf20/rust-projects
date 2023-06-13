fn main() {
    // Types can be explicit or implicit by the compiler
    let x = "Hello, world!";
    let y: u32 = 32000;

    // Fstring notation in rust
    println!("x: {}\n", x);
    println!("y: {}\n", y);

    // Mutable variables
    let mut z = 3;
    println!("z: {}\n", z);

    z = 1;
    println!("z: {}\n", z);

    // You can reassign variables
    let a = 1;
    println!("a: {}\n", a);
    let a = 2;
    println!("a: {}\n", a);

    // Constants, you have to define the type and value (cannot redefine like with "let" ^^)
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("SECONDS_IN_MINUTE: {}\n", SECONDS_IN_MINUTE);
}
