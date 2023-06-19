use std::io;

fn main() {
    // If confitions to set a variable.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    let input: &str = &input.trim();
    let greet: &str = if input.eq("Benjamin") {
        "Hello, Benjamin"
    } else {
        "Hello World!"
    };
    println!("{}", greet);
}
