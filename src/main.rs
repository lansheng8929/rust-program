use my_macro::CustomDebug;

// 自定义声明宏
macro_rules! my_println {
    // 匹配不带参数的调用（如 `my_println!()`）
    () => {
        println!()
    };
    // 匹配带参数的调用（如 `my_println!("Hello")`）
    ($msg:expr) => {
        println!("{}", $msg)
    };
}

#[derive(CustomDebug)]
struct Test {
    a: u32,
    b: String,
}

fn main() {
    let t = Test { a: 42,b: "Hello".to_string() };
    println!("{:?}", t)
}
