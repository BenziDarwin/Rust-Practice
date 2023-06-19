fn main() {
    // Used to implement debug function for struct person.
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        origin: String,
    }

    impl Person {
        // This is a constructor written for the Person struct.
        fn new(name: String, age: u32, origin: String) -> Self {
            Self {
                name: name,
                age: age,
                origin: origin,
            }
        }
        fn change_age(&mut self, new_age: u32) {
            self.age = new_age;
        }
    }

    // Needs to be declared as mutable for it's variables to be able to be edited.
    let mut benjamin = Person::new(String::from("Ssali Benjamin"), 20, String::from("Uganda"));
    println!("{}", benjamin.age);
    benjamin.change_age(34);
    println!("{}", benjamin.age);
    println!(
        "I am {} aged {} years from {}.",
        benjamin.name, benjamin.age, benjamin.origin
    );
    // How to print an instance of a struct in debug mode. We can use :? or :#?.
    dbg!("{:#?}", &benjamin); 

}
