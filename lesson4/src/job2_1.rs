enum MyEnum {
    Type1(u32),
    Type2(String),
    Type3(bool),
}

pub fn test2_1() {
    let vec: Vec<MyEnum> = vec![
        MyEnum::Type1(10),
        MyEnum::Type2(String::from("hello")),
        MyEnum::Type3(true),
    ];

    for item in vec {
        match item {
            MyEnum::Type1(val) => {
                println!("Type 1: {}", val);
                // 调用类型1的方法
            }
            MyEnum::Type2(val) => {
                println!("Type 2: {}", val);
                // 调用类型2的方法
            }
            MyEnum::Type3(val) => {
                println!("Type 3: {}", val);
                // 调用类型3的方法
            }
        }
    }
}
