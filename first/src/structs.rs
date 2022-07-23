#[derive(Debug)]
struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let mut user1 = User {
        user_name: String::from("someone"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("another@example.com");

    println!("{:#?}", user1);

    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
}

fn build_user(email: String, user_name: String) -> User {
    User {
        user_name,
        email,
        sign_in_count: 1,
        active: true,
    }
}
