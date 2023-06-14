// Rust does not care about the order of function definitions
fn test() {
    println!("test\n");
}

fn add(a: i32, b: i32) {
    println!("{} + {} = {}\n", a, b, a + b);
}

fn add2(a: i32, b: i32) -> i32 {
    a + b // Could also do return a + b; or return a + b
}

fn main() {
    test();
    add(20, 20);
    println!("{}", add2(20, 20));
}
