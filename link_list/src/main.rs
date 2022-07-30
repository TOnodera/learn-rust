enum Node {
    Empty,
    Cons(i64, Box<Node>),
}

use Node::{Cons, Empty};

fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}

fn main() {
    let c = node(10, node(20, node(30, Box::new(Empty))));

    let mut ptr = &c;
    loop {
        // ptr -> &Box<Node>
        // *ptr -> Box<Node>
        // **ptr -> Node
        // &**ptr -> &Node
        // ↓は↑の操作をやってる（Nodeの参照を取得してる）
        let cur_node = &**ptr;
        match cur_node {
            Empty => break,
            Cons(v, link) => {
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}
