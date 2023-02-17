//定义一个私有模块
mod back_of_house{
    pub struct Breakfast {
        pub toast:String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast:&str) -> Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches")
            }
        }
    }
}

// 通过暴漏公共方法来使用私有模块内的东西
pub fn eat_at_reastaurant(){
    let mut meat = back_of_house::Breakfast::summer("Rye");
    meat.toast = String::from("Wheat");
    println!("I'd like {} toast please", meat.toast);
}

//再定义一个公共mod
pub mod hosting {
    pub fn test_fn(){
        print!("test", );
    }
}

// 假设你在b里面用了一个a，并在b里面使用了pub use a::xx::yy ， 那么你在b里面可以直接使用a里的东西，同时把他暴漏给了c

// 如果要在同一个craft中引入多个内容，可以使用大括号的形式分别引入： use std::{cmp::Ordering, io};
// 或者要把std::io  和  std::io::Write  引入包中，可以使用Self引入，比如：use std::io::{Self, Write};
// 如果要引入路径下所有的公共项，应当使用：* ， 比如： use std::collections::*;  一般情况下我们不这么用

