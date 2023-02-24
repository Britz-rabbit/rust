//我们之前写了一个猜数字的游戏，现在我们复刻并改进一下
//之前我们在判断输入的时候，每次loop都会在内部作用域判断一次，现在我们尝试在外部定义一个结构体来实现相同的功能
pub struct Guess{
    val:u8,
}

impl Guess{
    fn new(val:u8) -> Guess{
        if val < 1 || val > 100 {
            panic!("输入范围应是1到100", );
        }
        Guess {val}
    }

    fn value(&self) -> u8{
        self.val
    }
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main(){
    
}