use std::num::{IntErrorKind, ParseIntError};

fn main() {
    /*
    This allows you to fix the error while using the match method.
       let val: &str = "99999999999999999999999999999999999999999999999999999";
       let num:i8 = match val.parse() {
           Ok(num) => num,
           Err(error) => match error.kind() {
               IntErrorKind::InvalidDigit => panic!("Invalid digit!"),
               IntErrorKind::Empty => panic!("Value is empty!"),
               some_error => panic!("Unknown error has occurred!: {:#?}", some_error)
           }
       };
    */
    // let val: &str = "99999999999999999999999999999999999999999999999999999";
    // //let num: i8 = val.parse().unwrap(); This calls the error directly.
    // // This gives you the option of handling the error within the code.
    // let num: i8 = val.parse().unwrap_or_else(|error: ParseIntError| {
    //     if error.kind() == &IntErrorKind::InvalidDigit {
    //         panic!("Invalid digit");
    //     } else {
    //         panic!("An error has occurred!{:#?}.", error)
    //     }
    // });

    let val: &str = "999999999999";
    let num: i8 = val.parse().expect("There is a positive overflow.");

}
