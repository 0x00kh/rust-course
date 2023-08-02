// 定义一个简单的声明宏，用于计算平方
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    let num = 5;
    let squared = square!(num);

    println!("{} squared is: {}", num, squared);
}