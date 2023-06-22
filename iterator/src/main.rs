

fn main() {
    let arr: Vec<i32> = vec![1,2,3,4];
    // Map is used to map through an iter and takes a function as a paramater. 
    // Collect is then used to turn the modified iter into the variable datatype.
    let arr1: Vec<i32> = arr.iter().map(|x: &i32| x +1).collect();
    assert_eq!(arr1, vec![2,3,4,5]);
    //We also have the filter method in an iter that returns a value that proves to be true for the value returned by the closure.
    // The into_iter method consumes the value rather than just referencing it.
    let arr2: Vec<i32> = arr.into_iter().filter(|x: &i32| x % 2 == 0).collect();
    dbg!("{:?}",arr2);
}
