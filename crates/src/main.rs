pub mod garden;

use garden::veggies::Carrots;
use std::collections::HashMap;

fn main() {
    let field1: Carrots = Carrots::new(String::from("West Side Field"), 34);
    println!(
        "Carrots are planted in the {}. The land in {}km2.",
        field1.field, field1.size
    );

    let text: &str = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        
        // This returns a mutable value to the reference.
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
