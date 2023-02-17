//三个在rust被广泛使用的集合是vector， string， hash map

//vector 1
/*
fn main() {
// let v:Vec<i32> = Vec::new();//使用Vec加泛型，通过Vec::new()来创建一个空泛型
let mut v = vec![1, 2, 3]; //或者使用vec![]的宏来快速创建

//更新这个vector
v.push(5);
v.push(6);
v.push(7);
v.push(8);

//使用索引语法获取vector内的元素
let third: &i32 = &v[2];
println!("the third is {}", third);
//使用get语法获取一个Option<&T>元素
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
//区别在于，直接访问索引出些错误时，会使rust panic；而第二种则会返回一个Option的None，并进行处理

let mut v = vec![1, 2, 3, 4, 5];
let mut first = &mut v[0];
v.push(6);
println!("The first element is: {}", first);
// 为什么第一个元素的引用会关心 vector 结尾的变化？不能这么
// 做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次
// 相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用
// 就指向了被释放的内存。借用规则阻止程序陷入这种状况。

let mut v = vec![1, 2, 3, 4, 5];//遍历一下
for i in &mut v{//使用&mut 来修改vector中的值
    // print!("{i} \n", );
    *i += 1;//使用*i接触引用，直接修改原本heap内存中i的值
}
println!("{:?}",&v );
} //离开作用于后，v会被释放
*/

//vector 2
/*
fn main(){
    //vector只能存储一种类型的数据，若想要存贮不同类型的数据，请结合enum一起使用
    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Float(3.6),
    SpreadSheetCell::Text(String::from("test"))
    ];

    //无论是提前规定整体vector的类型，还是通过enum来添加不同的类型，rust都需要提前知道存入的数据类型，这是处于高效利用存储的需要
}
*/

//string 1
//：Rust 倾向于确保暴露出可能的错误，字符串是比很多程序员所想象的要更为复杂的数据结构
// 但我们谈到rust的核心的时候，我们谈论的字符串指的是str，即字符串片段或者切片，通常以借用的形式出现，但是这种简单的数据类型是所见即所得的，可被轻易修改
// 而另一种情况，String，其实是由rust标准库提供的一种高级数据类型，它是可增长的、可变的、有所有权的、UTF-8 编码的一种集合
/*
fn main() {
    let mut s = String::new(); //创建一个空的字符串
    let mut s = String::from("text"); //利用shadow机制和字符串字面量来创建一个string

    // let str = "test str";//创建一个字符串切片
    // let string = str.to_string();//将这个str转变为string
    // let string = "test str".to_string();//上述两种方法合成一步
    // let string = String::from("test str");//作用上和to_string基本一致，不过官方更推荐这种风格。我觉得这样可读性更强一些

    // 更改string
    // s.push_str("bar");//对string类型使用push_str向后添加内容
    // println!("{s}", );
    // let s2 = String::from("bar");
    // // s.push_str(s2);//push_str不允许我们之间添加一个string，他必须要求是一个str。
    // s.push_str(&s2);//让我们使用&s2看看会怎么样
    // println!("{s}", );//我们 “用掉” s
    // println!("{s2}", );//可以看见，s2仍然可以使用（毕竟我们输入的是&s2嘛）

    // //另一种方式是push，这就是完全把string当作collections处理了,那么你也必须按照collecting的惯例--一次只添加一个单位
    // // s.push('bar');//error: character literal may only contain one codepoint
    // s.push('a');//你也可以理解为：push方法接受一个char作为参数
    // // s.push('\na');//error: character literal may only contain one codepoint
    // println!("{s}", );

    // // 来看看比较烧脑的部分
    // let s1 = String::from("hello ");//此处自带空格
    // let s2 = String::from("world");
    // let s3 = s1 + &s2;//我们定义了s3（注意：并没有定义s3的数据类型），并使用掉了s1和s2的引用
    // // println!("{s1}", );//error: borrow of moved value: `s1`   此时s1已经失去其使用权。被释放掉了
    // println!("{s2}", );
    // println!("{s3}", );
    // //当我们在使用 + 这个运算符的时候，我们实际上是调用了一个名为add的函数，add函数接受泛型参数
    // //在实现的时候，add要求的是一个string和str相加，但是，我们不是传入了一个string和一个&string吗？
    // // 这是因为在rust在调用add函数时，进行了强转（deref），帮我们把&string转成了&str，或者是，把&s2变成了&s2[..]，类似js的扩展运算符

    // // 当需要连接多个字符串时，add就显得有些繁琐，这时候推荐使用format宏
    // let s1 = String::from("hello");
    // let s2 = String::from("world");
    // let s = format!("{} - {}",s1,s2);
    // println!("{s}", );
}

*/

//string 2 关于字符串的索引及其他
/*
fn main() {
    //rust中不允许直接通过索引来获取string的元素
    // let t = s[0]; //error: the type `String` cannot be indexed by `{integer}`
    //为什么是{integer}? 原来，rust中的string其实是一组Vec<u8>的封装，并通过UTF-8实现
    //当我们写下s[0]这段代码时，rust应该返回的是UTF-8的编码值，但这并不是我们想要的，为了防止“请求一个字符却得到一个整数”的bug出现，rust根本不会编译这种代码
    //另一个原因是，通过索引访问的速度应当是(O(1))，但是rust本身并不能保证对string的操作达到如此的性能（比较rust是一门比较底层且注重性能的语言）

    //如果你通过索引来访问str，rust会允许你这么做，但是你得按照rust的规矩来
    //比较痛苦的一点是：你通过索引返回的元素，到底算是str?还是char?还是什么别的东西？
    // let hello = "Здравствуйте";
    // let s = &hello[0..4];
    // print!("{s}",); //Зд
    // let s = &hello[0..1];
    // print!("{s}",); //什么也没有，在早些时候甚至会产生panic。因为字母默认占用两个位置
    // //总之，不要轻易使用索引来访问字符串，即使是str

    // 尝试遍历一个chars
    for c in " नमस्ते".chars() {
        println!("{}", c);
    }

    //尝试以bytes的视角去遍历
    for b in " नमस्ते".bytes() {
        println!("{}", b);
    }
    //所以你明白为什么说字符串其实是一种Vector<u8>的封装了吧

    //     总而言之，字符串还是很复杂的。不同的语言选择了不同的向程序员展示其复杂性的方式。Rust 选择了
    // 以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更多的思考如何
    // 预先处理 UTF-8 数据。这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，不过也使你在开发
    // 生命周期后期免于处理涉及非 ASCII 字符的错误。

}
*/

//hash map 1
//HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
//它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
//很多编程语言支持这种数据结构，不过通常有不同的名字：哈希、map、对象、哈希表或者关联数组。
//听起来是和js里面的map有些相似，但是js的map可没有k，只有v。
// 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。

fn main() {
    use std::collections::HashMap; //用于hashMap不常用，所以它并没有像前两者那样被自动引入，我们需要手动添加它

    // //new一个新的hashmap并使用insert增加元素
    // let mut scores = HashMap::new();//依然是由于HashMap不常用，所以我们只能通过new的方式来创建，暂时没有字面量的宏创建形式
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Red"), 30);

    //    //法2：使用zip方法创建迭代器，并使用collect来进行集合转换
    //     let teams = vec![String::from("Blue"), String::from("Yellow")];
    //     let initial_scores = vec![10, 50];
    //     //通过注解类型为HashMap来告知collect转变为HashMap类型的collect
    //     //HashMap的k,v值的类型是需要手动指定的，如果你不确认k,v的数据类型，可以先使用_占位，之后再传入对应数据。
    //     let mut scores:HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // //HashMap的所有权：基本的简单数据会被直接copy，而string这类的复杂数据会被转移所有权
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name,field_value);//此时map获取了filed_name和filed_value的所有权
    // // println!("{filed_name}", );//error: cannot find value `filed_name` in this scope
    // //使用get方法来获取对应的值
    // let v = map.get("Favorite color");
    // println!("{:?}",v );//获取的值是一个Option   这行代码打印的：Some("Blue")

    //遍历map中的元素
    

}
