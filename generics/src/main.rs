struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn new(x:T, y:T) -> Self {
        Self { x: x, y: y }
    }

    fn set_values(&mut self, x:T, y:T) -> &str {
        self.x = x;
        self.y = y;
        "Values set successfully!"
    }
}


fn main() {
    let mut point = Point::<i32>::new(3,5);
    println!("{}", point.x);
    point.set_values(4,6);
    println!("{}",point.x);
}
