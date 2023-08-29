#[derive(Debug)]
struct User {
    username: String,
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
    // &selfとしないと、インスタンスの所有権が引数のselfに移ってしまい、インスタンス自体が使用できなくなる
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        username: String::from("taiyou"),
        email: String::from("taiyou.example.com"),
        sign_in_count: 1,
        active: true,
    };
    // インスタンスも同様に所有権が譲渡される
    let _user2 = user1;

    let mut user1 = User {
        username: String::from("taiyou"),
        email: String::from("taiyou.example.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.username = String::from("new_taiyou");
    user1.email = String::from("sun.example.com");
    user1.sign_in_count = 2;
    user1.active = false;
    println!("{:#?}", user1);

    let user2 = build_user(String::from("taichan.example.com"), String::from("taichan"));
    println!("{:#?}", user2);

    let rec = Rectangle::create(20, 20);
    println!("{:#?}", rec);
    rec.area();
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
