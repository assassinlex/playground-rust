use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    // println!("Hello, world!");

    let s = "hello world";
    let a = [1, 2, 3, 4, 5];

    // let s1 = String::from("hello");
    // let s2 = s1;
    // // 在编译期，上述操作会将 s1 无效化, 即从内存中删除，只保留 2
    // // println!("{}, world!", s1); // 错误示例: move操作(hello 已从 s1 move 到了 s2)
    // println!("{}, world!", s2); // 正确用法
}

/// 猜字游戏
#[allow(unused)]
fn guess_game() {
    println!("猜字游戏");
    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("生成随机数的值: {secret_number}");
    loop {
        println!("请输入您猜的数字:");
        // 获取用户输入
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");
        // println!("您的猜想: {}", guess);
        // 类型转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 结果匹配
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小"),
            Ordering::Greater => println!("大"),
            Ordering::Equal => {
                println!("正确");
                break;
            },
        }
    }
}
