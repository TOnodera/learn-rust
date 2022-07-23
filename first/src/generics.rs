use std::vec;

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("{}", largest(number_list));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largest(char_list));
}

fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
