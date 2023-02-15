//match 匹配模型
//在概念上和js的switch case或者py的match差不多，但是语法更加简洁

//试着用match和enum来编写一个验钞机吧
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_Cents(coin: Coin) -> u8 {
//     //这里我们设计了一个match来匹配Coin里面的枚举成员，并通过不同的成员返回不同的值
//     //事实上，返回值也可以是代码块，这些代码块里面可以执行逻辑，但不管怎么说，总要有返回值
//     match coin {
//         // Coin::Penny => 1,
//         Coin::Penny => {
//             println!("this value is 1", );
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

//我们也可以使用match来把两个枚举中的成员连接起来
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     //这样写，规定了Quarter里面包含了UsState的信息，在使用match语句时，UsState会作为state进入match判断
//     Quarter(UsState),
// }
// fn main() {
//     value_in_Cents(Coin::Quarter(UsState::Alaska));
// }
// fn value_in_Cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("You got a Quarter from {:?}", state);
//             25
//         }
//     }
// }

//使用match来操作Option<T>
// fn main(){
//     let five = Some(5);
//     fn plus_one(x:Option<i32>) -> Option<i32>{
//         match x {
//             None => None,
//             Some(i) => Some(i+1)
//         }
//     }
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

//一组常用的combo是 _ => ()   使用通配符完成match语句的穷举要求，再通过空元组处理。 “你完成了一套很牛逼的combo，但是无事发生。”
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (),
//     }
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
// }


//当情况分支很多时，即使是match语句也会显得有点烦。这时候你可以尝试使用if let
fn main(){
     let config_max = Some(3u8);
     //只有当Some(max)为某项值时，我们才会执行某些语句，其他情况我们就不管啦
     // 使用if let语法，可以避开match的穷尽性要求。你可以认为if let是match的某种语法糖
     if let Some(max) = config_max {
          println!("The maximum is configured to be {}", max);
     } 
     //如果你需要的话，你可以使用else来代替“很牛逼的combo”，不过这种情况并不常见        
}