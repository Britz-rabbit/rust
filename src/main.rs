use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); //调用生成器并生成随机数，获取随机数的参数形式为start..=end

    loop {
        println!("please input your number!");

        let mut guess = String::new(); //此处的::表示引用对应类型的关联函数

        io::stdin()
            .read_line(&mut guess) //返回一个Result类型的数据，由expect处理
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number"); //通过shadow来转换值的类型（用户收入回车会带进去一个换行符）

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };//在上述基础上通过match {}处理用户输入
            

        println!("You gressed:{guess}");

        match guess.cmp(&secret_number) {
            //传入参数是被比较的值，使用match对返回值做出不同相应
            Ordering::Greater => print!("Be smaller\n"),
            Ordering::Less => print!("Be biger\n"),
            Ordering::Equal => {
                print!("You win\n");
                break;
            }
        }
    }
}
