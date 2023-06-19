fn main() {
    let x = 6;

    // The values of a value inside scope don't affect the values out of the scope.

    // This is how you write a macro (It should return a value).
    let y = {
        let x = 3;
        println!("X inside scope is equal to {}.", x);
        x + fiver()
    };
    println!("X out of scope is {}", x);
    println!("Value of y is {}", y);
}

// This is how to declare a return value and return a value.
fn fiver() -> u32 {
    5
}
