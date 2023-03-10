//一，泛型 : 某一类数据类型或结构的占位符
//第一阶段探究：所有权的纠结
/*
fn largest_i32(list:&[i32]) -> i32{
    let mut largest = list[0]; //i32  通过引用权访问到了具体的数据
    // print_type_of(&list);//&[i32]
    for &item in list{//遍历的话会自动获取内部元素的地址值
        if item > largest {
            // 此时item是i32
            // print_type_of(&item);
            largest = item //此时遍历出来的是引用的数组内的一个简单类型的数据，他的copy方式是直接赋值，largest以此种方式拥有了一个属于自己的地址值
        }
    }
    // let addr = &largest as *const i32 as usize;
    // print!("{:X}",addr );//4180DBF78C100
    largest//出于rust语言的限制，你不能返回一个函数作用域内的变量引用值，因为它会在函数执行完的一瞬间被释放掉
           //所以你的返回值必须来源一个实际存在在内存中的值，它要么在栈内存有一片空间，要么在堆内存有一片空间
           //换言之，你返回的值必须有属于地址的地址值，或者它是引用外部的某项数据的地址值，才会被rust承认有效
           //所以我们以此为根本，向上逆推
}

fn main(){
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{}",largest_i32(&number_list) );
    // println!("{:?}",number_list );//100的所有权还在
    // let addr = &number_list[3] as *const i32 as usize;
    // print!("{:X}",addr);//1BC50EFEA7C
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
    }
*/

// 第二阶段探究：泛型能表明的信息有限
/*
fn find_largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        if i > largest {//error: binary operation `>` cannot be applied to type `T`
                        // T 可以表明的信息太少，不是所有的数据之间都可以使用 > 这个运算符的
            largest = i
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/

// 第三阶段探究：结构体与泛型
/*
struct Point<T>{//泛型一般用大写字母表示
    x:T,
    y:T
}

// 当你想使用多个类型时，应该定义两个泛型
struct OtherPoint<T,U>{
    x:T,
    y:U
}

//另一种实现多个类型的方法是定义一个枚举
//同一个枚举中的成员会被认为是同种类型，故可以用一个泛型代替
//最简单的例子应该就是Option的枚举类型了
enum Option<T>{
    Some(T),
    None
}

enum PointEnum{
    int(i32),
    float(f32)
}

fn main(){
    let int_point = Point{
        x:5, y:10
    };

    let float_point = Point{
        x:5.0, y:10.0
    };

    let int_and_float_point = OtherPoint{
        x:5, y:10.0
    };

    let enum_point = Point{
        x:PointEnum::int(5),
        y:PointEnum::float(10.0)
    };

}
*/

// 第三阶段探究：泛型与方法
/*
//定义泛型做参数的结构体
struct Point<T>{
    x:T,
    y:T
}

//方法也使用泛型做参数
impl<T> Point<T> {//第一个T是impl声明的.这样说吧,impl也是一个函数,传入的参数是泛型T,而point里面的T必须来自外部impl传入
    fn test(&self) -> &T{//传入Point的实例本身
        &self.x
    }
}

//方法使用具体的数据类型做参数
impl Point<f32> {//你可以不用在使用impl函数的时候传入泛型,而是让内部的函数使用rust里的数据类型,
                 //但是以此方式创建方法只能被用作特定的结构体实例
    fn distance_from_orgain(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//处于需要,我们构建另一个结构体,接受两个泛型参数
#[derive(Debug)]
struct Point2<T,U>{
    x:T,
    y:U
}

//使用泛型做参数的方法来返回一个新的结构体实例

impl<T,U> Point2<T,U> { //我们impl函数调用时传入了T和U两个泛型参数
    //注意:此时我们传入的是self,而非&self,如果我们传入&self下面返回一个Point2的时候会报错.
    fn minup<T2,U2>(self, OtherPoint:Point2<T2,U2> ) -> Point2<T,U2>{//我们在构造minup方法的时候,需要重新设定两个泛型参数,即T2和U2
        // let  x = self.x;// error:move occurs because `self.x` has type `T`, which does not implement the `Copy`
        Point2{
            x:self.x, // label: move occurs because `self.x` has type `T`, which does not implement the `Copy`
                      // 看起来像是泛型T不能应用copy方法,不过报错却是self的引用值不能被共用
                      // 在下方的测试中发现, 此时我们传入的Point2<T,U>和Point2<T2,U2>的所有权均已消失, 转而到了返回的新实例Point<T,U2>身上
                      // 另一个原因就是,我们需要的是一个Point2, 是一个活的, 独立的结构体实例
            y:OtherPoint.y
        }
    }
}

fn main(){
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.test());

    let q = Point{x:5.0 , y:10.0}; //创建一个T为f32的实例,这样我们定义的第二个方法才能在此应用
    println!("the distance of the point from the orgain is {}", q.distance_from_orgain());

    // 为了测试point2的方法对引用权的改变,我们实例化两个Point2
    let p2 = Point2{ x:5.0, y:10 };
    let q2 = Point2{ x:5, y:10.0 };
    let test = p2.minup(q2);
    // println!("p2 is {:?}", p2); //测试结果:p2的所有权已经改变  error: borrow of moved value: `p2`
    // println!("q2 is {:?}", q2); // label: value borrowed here after move
    // println!("test is {:?}", test);
}

// 补充一点:使用泛型可以使代码更加简便和明晰,却不会带来性能上的任何损失,这是由于rust在编译时进行了名为 '单态化' 处理
// 单态化: 是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
*/

//二，trait : 一种定义共享行为的抽象方式
// 第一阶段: trait基础写法
/*
pub trait Summary {
    fn Summary(&self) -> String; //每个方法签名都需要以分号结尾
                                 //每个方法签名不提供具体实现
                                 //编译器会确保Summary trait的类型都拥有复合内部签名定义一致的方法
}

//定义结构体A
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//将trait应用到结构体A上
impl Summary for NewsArticle {
    fn Summary(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

//定义结构体B
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//将trait应用到结构体B上
impl Summary for Tweet {
    fn Summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main(){

}
//补充:对于两个不同的包, 其同名的trait不会相互干扰
*/

// 第二阶段探究: trait的默认实现
/*
pub trait Summary {
    fn summarize(&self) -> String {
        //在定义时,直接将其实现写出来(无需在之后加分号)
        String::from("(Read More)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//对NewsArticle使用默认的summary
impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//对Tweet使用自定义的summary
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    //默认实现
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    //自定义实现
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
*/

// 第三阶段探究:给参数使用trait
/*
//先声明两个trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Display {
    fn display(&self) -> String;
}

pub trait Test {
    fn test(){
        print!("test trait", );
    }
}
//声明结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// pub fn notify(item: &impl Summary) {
//     //我们给notify这个函数的参数item用了trait,此时在notify的作用域中可以使用来自trait定义的行为,并受其约束
//     // impl trait的写法实际上是一个语法糖, 其本来是一种被称为trait bound的语法,顾名思义,就是使用trait来设立bound
//     println!("Break news! {}", item.summarize());
// }

// //trait bound
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// //当参数超过两个后,使用trait bound效果会更好一些
// pub fn notify<T:Summary>(item_1:&T, item_2:&T){//还记得这种写法吗?通过在外部传入一个泛型T,内部的参数在使用T或T的引用值.
//                                               //参考上方泛型与方法相关部分
//     println!("Breaking news! {}", item_1.summarize());
//     println!("Another breaking news! {}", item_2.summarize());
// }

// //当你要一次指定多个trait bound的时候,可以使用 + 来指定
// pub fn notify<T:Summary + Display>(item:&T){

// }

// 当使用的泛型过多时,可以使用 where 进一步简化
fn some_fn<T,U>(t:&T,u:&U) ->i32
    where T:Display + Clone, //使用这种签名能更清晰地说明各泛型参数对应的trait bound
          U:Clone + Test
{
    6
}

fn main() {
    let q = Tweet {

        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know"),
        reply: false,
        retweet: false,
    };

    let q2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know"),
        reply: false,
        retweet: false,
    };

    // notify(&q);
}
*/

// 第三阶段探究:给返回值使用trait
/*
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 下方写法会报错 label: missing `summarize` in implementation
// impl Summary for Tweet {
//     print!("test", );
// }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn return_summarizable() -> impl Summary {
    //给返回值进行约束:需要使用 Summary 的trait
    //如果返回值没有impl 名为 Summary 的trait, 则无法通过编译
    //这个功能在闭包和迭代器的相关场景会比较有用
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
//闭包和迭代器创建只有编译器知道的类型，或者是非常非常长的类型。
//impl Trait 允许你简单的指定函数返回一个 Iterator 而无需写出实际的冗长的类型。
//实际使用者可能不会确定只返回一个Tweet,应该是经过判断再确定要返回的类型

// 在上方泛型第二阶段探究中,我们写下了 if i > largest 这行代码,但是未能通过编译,原因在于不是所有的泛型都可以被比较大小
// 事实上,当我们使用 > 这个运算符的时候,调用的是trait std:: cmp::PartialOrd 的一个默认方法。而标准库是被prelude默认导入的,我们可以直接调用
// 而这个用于比较的trait，其规定的类型有限(更专业的说法是:签名)
// 另一个问题是，所有权的问题。我们之前见过这个问题，某个属性不能应用copy
// 要想跑通整个过程，我们需要给参数的泛型实现 PartialOrder 和 Copy 两个 trait， 如下：

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {//copy可以处理堆内存，若只需要处理栈内存，请使用Clone trait
    let mut largest = list[0];

    for &i in list {
        //就直接记住吧，不想再推了
        if i > largest {
            largest = i
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
//通过trait， Rust 将可能在运行时出现的问题移动到了编译
// 接下来我们来介绍另一种特殊的泛型，生命周期（lifetimes）
*/

//三，生命周期：引用和借用保持有效的作用域
// 第一阶段探究：生命周期避免了垂直引用
/*
//一个典型的引用错误
// fn main(){
//     {
//         let x;
//         {
//             let y = 5;
//             x = &y;//y的引用值在超出y的作用域后就失效了//label: borrowed value does not live long enough
//         }
//         println!("x is {}", x);//此时我们在使用&y就会报错
//     }
// }

//通常来说，一个变量的作用域越大，我们说它的存的越久
//内部是借助rust的借用检查器(borrow checker)来实现的，
//在检查编译的时候，borrow checker会检查两个生命周期的大小，
//发现较小的生命周期值的值被较大生命周期给引用了，就会拒绝编译
*/

// 第二阶段探究：生命周期与泛型
/*
//需求：比较两个字符串，并返回较长的一个

// fn longest(str1:&str,str2:&str) -> &str{ //error: missing lifetime specifier
// 编译失败，提示信息告诉我们需要一个揭示生命周期的标识（这实际上是一个泛型参数）
// 这是由于rust并不清楚要返回的值对应的生命周期，不知道如何看待str1和str2的生命周期
// borrow checker不知道给str1和str2分配什么值
//     if str1.len() > str1.len(){
//         &str1
//     } else{
//         &str2
//     }
// }

// 要修改上述函数，我们需要使用生命周期注解语法。
// 这个语法本身不会对任何引用的生命周期产生影响，只是帮助编译器更好的分析
// 当涉及多个引用的时候，生命周期注解能更好的描述多个引用生命周期的相互关系
// 其语法是'a 并在后面一一个空格跟引用隔开，例如 &'a mut i32

// //声明周期注解的语法就算泛型的语法：两个尖括号， 声明周期注解是一种特殊的泛型
// fn longest<'a>(s1:&'a str, s2:&'a str) -> &'a str{
//     if s1.len() > s2.len(){
//         s1
//     } else {
//         s2
//     }
// }
// // 虽然生命周期注解本身不会改变代码的引用权，但是编译器会遵守注解的限制，并给出我们更多提示
// // 生命周期注解实际上是取到任何应用了注解的引用的生命周期的最小值，即服从短板定理

// fn main(){
//     // let s1 = 'abcd';//报错 ''只能用于char，不能用于str
//     let s1 = "abcd";
//     let s2 = "xyz";
//     let result = longest(&s1, &s2);
//     println!("the longest is {}", result);
// }

// 如果你将main里面写成了下面这个样子，可能就不大合适了
fn main() {
    let string1 = String::from("this is a long string");
    let result;
    {
        //我们在此处的生命周期和函数调用时的生命周期并没有什么联系
        let string2 = String::from("it's a short string");
        result = longest(string1.as_str(), string2.as_str()); //error: `string2` does not live long enough
                                                              //这是由于我们在longest中给str1和str2给了相同的生命周期注解，而此时我们的string1和stirng2的周期不一样长，所以才会报错
        println!("the longest string is {}", result);
    }
}

//对于此种情况，我们需要给str1和str2不同的生命周期注解
fn longest<'a, 'b, 'r>(s1: &'a str, s2: &'b str) -> &'r str
where
    'a: 'b,
    'b: 'r,//将输入值设为g，代表global，实际上是不合适的行为，所有我把它改成了r，它仅仅代表result
{
    //返回值标准生命周期注解
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
*/

// 第三阶段探究：生命周期与结构体
/*
struct MyStruct<'a> {
    //这个注解做出的限制是：MyStruct的实例不能比part的引用存在的更长（短板效应）
    part: &'a str,
}

fn main() {
    let novel = String::from("Long long time ago, there is a pig standing on the floor.");
    let first_stence = novel.split('.').next().expect("Colud not find a ','");
    // first_stence所拥有的生命周期值较大，所以可以被小的作用域引用
    let i = MyStruct { part: first_stence };
    println!("the first word is {}",first_word(&novel.as_str()));
}
//注意：如果输入的生命周期参数里面有一个是&self或者&mut self，则说明这是一个对象的方法，此时所有的生命周期参数都应当被赋予self的生命周期

// 我们之前写了一个函数
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();//我们输入一个字符切片，将他分成字节
    for (i, &item) in bytes.iter().enumerate() {//将字节变成可迭代数组并进行遍历
        if item == b' ' {//在出现单词边界的时候停止，找到对应的索引值
            return &s[0..i];//返回输入字节对应的索引之前的所有部分，即一个单词
        }
    }
    &s[..]
}
// 上述函数并没有使用生命周期注解，却能通过编译。原因是这里的生命周期其实是默认实现的

//补充：生命周期与方法
// 这里的方法是与结构体对应的，当一个结构体被标注生命周期后，其impl的方法也需要考虑生命周期的影响
struct MyStruct<'a>{
    part:&'a str,
}
// 给方法加入生命周期注解的写法是紧跟在impl关键字之后，如下
// 这个方法的唯一一个参数是&self，而返回值与其无关，换句话说，这个返回值不牵扯任何引用
impl<'a> MyStruct<'a>{//不止impl关键字和结构体名后面都要加生命周期注解
    fn show_level(&self) ->i32 {
        3
    }
}

fn main(){

}
*/

// 第四阶段探究： 静态生命周期
/* 
// 静态生命周期是一种特殊的生命周期，其可以存在与整个程序
// In another word, it can live as long as script goes.
// 所有的字符串自变量都拥有静态生命周期，标注是'static
fn main(){
    //被static标注的数据会被直接硬编码到二进制文件中
    let  s:&'static str= "I got a long life as program do";
    // 实际上，&'static T是一个指向某个T的不可变引用，能够被安全持久的存储
    // 另一种： T:'static的写法是另一种能安全持久的储存，T:'static 包含了所有的  &'static T，并且可以无限的持有他们
    // T:'static 实际上表示的是T被static lifetime约束的概念
}
// 最后：慎用'static ， 因为它会一直存在于你的程序中，直到永远...永远....
*/

//综合样例：泛型， trait bound， 生命周期
use std::fmt::Display;
fn main(){
    let string1 = String::from("this is a long string");
    let string2 = String::from("it's a short string");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Today is a good day!");
    println!("{}",result );
}

fn longest_with_an_announcement<'a,T>(
    x:&'a str,
    y:&'a str,
    ann:T
) -> &'a str
where T:Display
{
    println!("Announcement!{}",ann);
    if x.len()>y.len(){
        x
    } else {
        y
    }
}

// 泛型类型参数意味着代码可以适用于不同的类型。
// trait 和 trait bounds 保证了即使类型是泛型的，这些类型也会拥有所需要的行为。
// 由生命周期注解所指定的引用生命周期之间的关系保证了这些灵活多变的代码不会出现悬垂引用。
// 而所有的这一切发生在编译时所以不会影响运行时效率！