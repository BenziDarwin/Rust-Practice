fn main() {
    // The first numerical value of an enum is 0.
    enum DaysOfTheWeek {
        Monday,
        Tuesday,
        Wednesday
    };

    let day:DaysOfTheWeek = DaysOfTheWeek::Monday;
    // We can add values to an enum value.
    #[derive(Debug)]
    enum Triangle {
        x(i32,i32,i32),
        y(i32,i32,i32),
        z(i32,i32,i32),
    }

    // This is one way to access constants in 
    if let Triangle::x(x ,_,_) = Triangle::x(2, 4, 7) {
        println!("{}", x);
    };

    struct Rectangle {
        length:i32,
        width:i32
    }

    impl Rectangle {
        fn new(length:i32,width:i32) -> Self {
            Self { length: length, width: width}
        } 

        fn calcluateArea(&self) -> i32 {
            self.length * self.width
        } 
    }

    enum Shape<T, R> {
        name(T, R),
    }

    let shape = Shape::<i32, i64>::name(32,12);
}
