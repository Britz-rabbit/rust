// 枚举是通过列举可能的成员来定义一个类型的，

// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn main() {
//     // 使用 :: 来去某个枚举内寻找成员，这样相同的成员名可以在不同的枚举类中使用，不会混淆
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     // 此时IpAddrKind会被当成一种特殊的数据结构来在代码中使用，并且其内部的成员也会被视作相同类型
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }
// //println!("{:?}",ip_kind );
// fn route(ip_kind: IpAddrKind) {}

//
// 接下来我们将枚举和结构体结合起来使用
// fn main(){
//     enum IpAddrKind{
//         V4,
//         V6
//     }
//     struct IpAddr {
//         kind:IpAddrKind,
//         address:String
//     }
//     let home = IpAddr{
//         kind:IpAddrKind::V4,
//         address:String::from("127.0.0.1")
//     };
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//         };
// }

//对于上述的用法，有着另一种书写方式，更加简洁
// fn main(){
//     enum IpAddrKind{
//         //直接将数据原始实例写道枚举成员内，在使用时直接使用对应的方法赋值即可
//         V4(String),
//         V6(String)
//     }
//     //这样我们便无需使用结构体额外规定adress了
//     //可以看到，我们只有定义的结构体成员是可以直接作为一个函数一样使用的，使用它，自动构造函数成员的值
//     //另一个好处是，你可以将V4设置为Number类型，V6设置为String类型，但他们都属于ipAddrKind的成员
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));
//     let loopback = IpAddrKind::V6(String::from("::1"));
// }

// 除去无关联类型的枚举成员，有关联类型的枚举成员，枚举成员还能是一个类结构体的写法，
//同时枚举类也能想结构体一样被引用一个方法
// fn main(){
//     enum Message{
//         Quit,
//         Move {x:i32 ,y:i32},
//         Write(String),
//         ChangeColor(i32, i32, i32)
//     }
//     impl Message {
//         //这里我们依然传入引用
//         fn call(&self){
//             println!("test", );
//         }
//     }
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// 接下来我们看看Option，它是rust标准库定义的另一个枚举，代表有值或没有值。
//在rust中，Null是一个特殊的值，这与其他许多编程语言不大一样，所以需要用另一个东西来代表是不是有值，这样做是为了避免一些bug
//空值的概念是有意义的，它表示暂时确实或无效的值，而其他编程语言的Null在表达这一概念的时候会出现很多问题
// fn main(){
//     let some_number = Some(5);
//     let some_char = Some('e');
//     //这里的<T>是泛型参数的概念，ts里面也有这种语法。这里我们定义了一个None，而它原本应该是一个i32类型的数据
//     let absent_number:Option<i32> = None;
// }

//当我们视图运行以下代码的时候，会报错，因为Option无法被像一个具体的数据一样使用
//就像ref不能和具体的值相加一样，你应该用ref.value和具体值的值做运算
//运用Option，你可以做出这样的设计：当某个值存在时，它是具体的值；当某个值不存在时，它是Option<T>，而不会是null或者报错
//总而言之，Option是为了阻止空值的泛滥以及据此造成的不安全性而设计的
//具体的使用方法要根据设计场景来实现，这里就先不展示了，我们之后再看
fn main(){
    let x:i8 = 5;
    let y:Option<i8> = Some(5);
    // let sum = x + y;
}