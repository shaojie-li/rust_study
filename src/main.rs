use rand::Rng; // prelude，标准库中的io库
use std::cmp::Ordering;
use std::io; // trait

// 入口函数
fn main() {
    // println + !，表示一个宏，将文字输出到屏幕上
    println!("猜数游戏！");

    let secret_number = rand::thread_rng().gen_range(1, 101); // i32(默认) u32 i64...

    println!("神秘数字是{}", secret_number);

    loop {
        println!("猜测一个数");

        // let mut foo = 1;
        // let bar = foo;
        // foo = 2;
        // mut表示可变的变量；下面返回一个可变的String类型的空字符串实例
        let mut guess = String::new(); // utf8编码
                                       // io标准库，下面有一个stdin库。
                                       // read_line方法可以获取用户的输入
                                       // & 取地址符号，表示是一个引用类型的变量，可以在不同的地方访问同一块内存
                                       // mut表示可变的变量
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // 返回 io:Result OK, Err，枚举类型
        // 如果io:Result 返回的值为Err，expect方法就会中断当前程序，并将传入的字符串信息显示出来
        // 如果io:Result 返回的值为Ok，expect方法就会提取出Ok中附加的值，并将这个值作为结果返回的用户
        // trim - 去掉空白，空行，回车
        // parse - 解析成指定类型，下面为u32（无符号整数类型）
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是：{}", guess);
        // cmp 表示比较 -> compare
        // match 匹配到其中到分支，则执行匹配的分支
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
