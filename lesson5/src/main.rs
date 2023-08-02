// 声明宏 `my_macro`，它将传入的表达式封装在一个闭包中，并返回闭包的结果
macro_rules! my_macro {
    ($x:expr) => {
        {
            let y = $x;
            || y * 2
        }
    };
}

fn main() {
    let closure = my_macro!(5);
    println!("Result: {}", closure());
}