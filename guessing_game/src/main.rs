use std::collections::HashMap;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let res = largest(&number_list);
    println!("The largest number is {res}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let res = largest(&char_list);
    println!("The largest char is {res}");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
