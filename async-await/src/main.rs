use futures::executor::block_on;
use futures::join;

async fn hello_world() {
    println!("hello world");
}
async fn hoge_fuga() {
    println!("hoge fuga");
}

async fn run() {
    join!(hello_world(), hoge_fuga());
}
fn main() {
    block_on(run());
}
