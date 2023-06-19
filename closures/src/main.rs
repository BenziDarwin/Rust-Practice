#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColors {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColors>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColors>)  -> ShirtColors {
        //A closure is an anonyumous function.
        // This is a closure implemented in the unwrap_or_else function.
        //It is like a lambda function that can be denoted with; || {function body}
        user_preference.unwrap_or_else(|| self.make_preference())
    }

    fn make_preference(&self) -> ShirtColors {
        let mut num_red = 0;
        let mut num_blue = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColors::Red => num_red += 1,
                ShirtColors::Blue => num_blue +=1,
            }
        }
        if num_blue > num_red {
                ShirtColors::Blue
            } else {
                ShirtColors::Red
            }
    }
}

fn main() {
    let store = Inventory {shirts: vec![ShirtColors::Blue, ShirtColors::Blue, ShirtColors::Blue, ShirtColors::Red]};
    let giveaway1 = store.giveaway(Some(ShirtColors::Red));
    println!("User with some pref {:?} get {:?}", Some(ShirtColors::Red), giveaway1);
    let giveaway2 = store.giveaway(None);
    println!("User with some pref {:?} get {:?}", None::<ShirtColors>, giveaway2);
    //Closures are called like normal functions.
    let add_one_v4 = |x| -> i32  {x + 1};
    println!("{}",add_one_v4(3));

}
