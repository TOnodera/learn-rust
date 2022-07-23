pub fn run() {
    let st1 = String::from("x");
    let res1;
    {
        let st2 = String::from("y");
        res1 = get_longest(&st1, &st2);
        println!("{}", res1);
    }
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
