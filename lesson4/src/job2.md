使用枚举类型包裹三个不同的类型的例子如下所示：

```rust
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
```

使用Trait Object将三个不同类型放入Vec中，并对其进行遍历调用各自的方法的例子如下所示：

```rust
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
```


###### 枚举类型包裹不同的类型
使用枚举类型可以在定义时明确规定所有可能的类型，通过匹配枚举值可以调用不同类型的各自方法。这种方法的优势是代码结构简单直接，适用于已知的类型集合且类型数量较少的情况。


######  使用Trait Object
使用Trait Object可以将不同的类型放入相同的容器中，通过调用Trait中定义的方法实现多态。这种方法的优势是可以方便地添加新的类型，扩展性好，适用于类型未知或类型数量较多且需要灵活扩展的情况。不过，使用Trait Object也会带来性能的损失，因为需要进行动态分发。

