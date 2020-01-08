fn main() {
    println!("Hello, world!");

    let v = vec![1,2,3,3,4,5,6];

    let result = largest(&v);

    println!("largest : {}", result);
}

fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}