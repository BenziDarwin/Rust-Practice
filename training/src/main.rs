
fn main() {

    // Tuples
    let tup:(i64,f32,u8) = (34,2.03,1);
    let (x,y,z) = tup;
    println!("The value of x is {}, y is {}, z is {}.", x,y,z);
    // Tuples can also be accessed through indexes.
    println!("Acceessing first value via index, {}.", tup.0);
    
    //Arrays
    // You can't access elements out of bounds of the array.
    let array:[i64;5] = [1,2,3,4,5];
    println!("This is the 4th element in the array, {}.", array[3]);
    // You can use array.len() to get the length of the array.
    println!("{} is the length of the array.", array.len());
    greeter("Benjamin");

    
}


fn greeter(name:&str) {
    println!("Hello, {}", name);
}
