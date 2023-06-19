pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct  Rectangle {
    w: i32,
    h: i32
}

impl Rectangle {
    fn new(w: i32, h:i32) -> Self {
        Self { w: w, h: h }
    }

    fn can_hold(&self,x: i32, y:i32) -> bool {
        self.w > x && self.h > y 
    }
    fn can_panic(&self){
        panic!("I want to panic!");
    }
}

#[cfg(test)]
mod tests {
    // This is used to use the functions and other values out of this scope.
    use super::*;

    #[test]
    fn test_can_hold_smaller() {
        let rect = Rectangle::new(10, 5);
        assert_eq!(rect.can_hold(9, 4), true);
    }
    #[test]
    fn test_can_hold_larger() {
        let rect = Rectangle::new(10, 5);
        // This checks for equality, assert_ne checks for inequality, and passes if they are not equal to each other.
        assert_eq!(rect.can_hold(11, 6), false);
    }

    #[test]
    // #[ignore] This can be used to ignore a specific test. We can used the -- --ignored to get ignored messages.
    // The expected is a substring that should be in the panic message.
    #[should_panic(expected = "want to panic")]
    fn test_can_panic() {
        let rect = Rectangle::new(10, 5);
        rect.can_panic();
    }

    #[test]
    // Result returns ok and err values
    fn test_result() -> Result<() , String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("Not equal to value!"))
        }
    }
}
