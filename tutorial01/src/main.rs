fn main() {
    // Types can be explicit or implicit by the compiler
    let x = "Hello, world!";
    let y: u32 = 32000;

    // Fstring notation in rust
    println!("x: {}\n", x);
    println!("y: {}\n", y);

    // Mutable variables, variables are immutable by default
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

    // Data types -> Scalars and Compound (Array, tuple)
    // Scalars
    let x: i32 = 1; // Integer, default value for integers is i32 (i8, i16, i32, i64, i128)
    println!("x: {}\n", x);
    let x: u32 = 1; // Unsigned integer (u8, u16, u32, u64, u128)
    println!("x: {}\n", x);
    let x: f32 = 1.0; // Float, default value is f64 (f32, f64)
    println!("x: {}\n", x);
    let x: bool = true; // Boolean (true, false) could be also 0 and 1
    println!("x: {}\n", x);
    let x: char = 'a'; // Single character (char)
    println!("x: {}\n", x);

    // Compound types
    let x: (i32, f64, u8) = (500, 6.4, 1); // Tuple, cant be uninitialized possible? similar to arrays
    println!("1: {}, 2: {}, 3: {}\n", x.0, x.1, x.2);

    let x: [i32; 3] = [1, 2, 3]; // Array, cant be uninitialized and has a fixed length,
                                 // cant assign [] to a variable
    println!("x[0]: {}\n", x[0]);

    let x: &str = "Hello, world!"; // String
    println!("x: {}\n", x);
}
