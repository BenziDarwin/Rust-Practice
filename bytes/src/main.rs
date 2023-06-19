fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);

    // Here is an example of slicing in rust.
    let firstword = &s[0..word];
    println!("{} is the first word.", firstword)
 
}

fn first_word(statement: &String) -> usize {
    // Word needs to be converte to bytes before being used in a for loop.
       for (i, item) in statement.as_bytes().iter().enumerate() {
        if item == &b' ' {
            return i;
        }
    }
     statement.len()
} 
