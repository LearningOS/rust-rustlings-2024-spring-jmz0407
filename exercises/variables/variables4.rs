// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut x = 3; //rust 变量默认是不可改变的，若要使变量可变，需要在声明时使用mut关键字
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
