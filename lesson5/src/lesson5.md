main.rs中定义了一个名为square的声明宏，它接受一个表达式作为参数，并返回该表达式的平方。在main函数中，我们使用了这个声明宏来计算一个数字的平方并进行打印。

关于编译过程，Rust 的宏在编译时被展开。当编译器遇到宏调用时，它会根据宏定义的代码生成等效的 Rust 代码，并将其插入到调用位置。