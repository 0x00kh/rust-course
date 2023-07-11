fn test1() {
    // 创建一个 String 类型的值，并将其赋值给变量 s
    let s = String::from("hello");

    // 将变量 s 的所有权转移给变量 s2
    let s2 = s;

    // 这里编译器会报错，因为 s 的所有权已经转移给了 s2
    // println!("{}", s);

    // 可以使用 clone() 方法来克隆具有堆分配数据的值，创建一个新的值和一个新的所有者
    let s3 = s2.clone();

    println!("s2 = {}, s3 = {}", s2, s3);
}

fn test2() {
    let s = String::from("hello");

    // 创建一个不可变引用
    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);

    // 不能同时存在可变引用和不可变引用
    // let r3 = &mut s;
}

fn test3() {
    let mut s = String::from("hello");

    // 创建一个可变引用
    let r1 = &mut s;

    // 只能存在一个可变引用
    // let r2 = &mut s;

    r1.push_str(", world!");

    println!("{}", r1);
}

fn main() {
    test1();
    test2();
    test3();
}
