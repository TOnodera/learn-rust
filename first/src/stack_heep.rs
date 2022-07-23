pub fn run() {
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("v1 :{:p}", &v1);
    println!("v2 :{:p}", &v2);

    println!("v1 heep: {:?}", v1.as_ptr());
    println!("v2 heep: {:?}", v2.as_ptr());

    println!("v1 len:{:?}", v1.len());
    println!("v2 len:{:?}", v2.len());

    v1.insert(1, 10);
}
