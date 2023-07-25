use std::ops::Add;

// 定义一个类型
struct MyType<T> {
    value: T,
}

// 实现 Add trait
impl<T: Add<Output = T>> Add for MyType<T> {
    type Output = MyType<T>;

    fn add(self, other: MyType<T>) -> MyType<T> {
        MyType {
            value: self.value + other.value,
        }
    }
}

// 定义一个泛型函数来执行加法运算
fn add_numbers<T: Add<Output = T>>(val1: MyType<T>, val2: MyType<T>) -> MyType<T> {
    val1 + val2
}

pub fn test1() {
    let num1 = MyType { value: 10 };
    let num2 = MyType { value: 20 };
    let result = add_numbers(num1, num2);
    println!("Result: {}", result.value);
}
