use std::{thread, time};

fn main() {
    println!("--スレッドなし---");
    sleep_print("スレッドなし");

    println!("--スレッドを利用--");

    thread::spawn(|| {
        sleep_print("次郎");
    });

    thread::spawn(|| {
        sleep_print("三郎");
    });

    sleep_print("太郎");
}

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
