struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn test1() {
    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user1 = User {
        active,
        username,
        email,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.email);
    println!("{}", user2.username);
}


struct MyStruct;

impl MyStruct {
    fn consume(self) -> u32 {
        // 使用 self，销毁 self
        42
    }
}

fn test2() {
    let my_struct = MyStruct { /* ... */ };
    let answer = my_struct.consume();
    println!("{}", answer);
    // let answer2 = my_struct.consume();
    // println!("{}", answer2);
}