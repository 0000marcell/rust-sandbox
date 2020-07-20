use std::fmt::Display;

fn largest<T: PartialOrd + Clone + Display>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    let mut i = 0usize;
    loop {
        let item = list[i].clone();
        if item > largest {
            largest = item;
        }
        if i == list.len() - 1 {
            break;
        }
        i += 1;
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
