use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("やあスレッドから立ち上げた数字{}だよ。", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("メインスレッドから数字{}だよ。", i);
        thread::sleep(Duration::from_millis(1));
    }
}