use std::io;
use rand::Rng;

fn main() {
    // println!("Hello, world!");
    println!("猜字游戏");
    println!("请输入您的猜想:");

    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("生成随机数的值: {secret_number}");
    
    // 获取用户输入
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("读取输入失败");
    // println!("您的猜想: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering:Less
    }
    
}
