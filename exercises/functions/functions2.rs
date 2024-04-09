// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

//rust中不需要事先声明函数，可以直接调用函数
//函数名全小写字母，单词之间使用下划线分割
//常量全大写字母，单词之间使用下划线分割

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
