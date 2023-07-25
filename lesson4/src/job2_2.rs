trait MyTrait {
    fn do_something(&self);
}

struct Type1(u32);
impl MyTrait for Type1 {
    fn do_something(&self) {
        println!("Type 1: {}", self.0);
        // 类型1的方法实现
    }
}

struct Type2(String);
impl MyTrait for Type2 {
    fn do_something(&self) {
        println!("Type 2: {}", self.0);
        // 类型2的方法实现
    }
}

struct Type3(bool);
impl MyTrait for Type3 {
    fn do_something(&self) {
        println!("Type 3: {}", self.0);
        // 类型3的方法实现
    }
}

pub fn test2_2() {
    let vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(Type1(10)),
        Box::new(Type2(String::from("hello"))),
        Box::new(Type3(true)),
    ];

    for item in vec {
        item.do_something();
    }
}
