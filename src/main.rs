mod fibonacci;
mod rectangle;

fn main() {
    // fibonacci
    println!("Fibonacci:");
    fibonacci::fibonacci(10);
    println!();

    // rectangle
    println!("Rectangle:");
    rectangle::main();
    println!();
}
