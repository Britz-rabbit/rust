fn main() {
    //str_string();

    // let str = String::from("您吃了吗");
    // let length = calculate_length(&str);
    // println!("the length of {str} is {length}", );

    // let mut str2 = String::from("hello");
    // mutable_reference(&mut str2);
    // println!("{str2}",);

    let mut str = String::from("hellow world");
    let res = first_word(&str);
    str.clear();//将可变的字符串清楚为" "
    print!("{res}", );//理论上讲，res是针对str的结果，现在我们把str清除了，res不受影响。则不算bug，但是有点不合适
}

fn str_string() {
    // String存于heap中，可以被修改，也正因此需要被存入heap中
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s is {}", s);

    // move与clone：move会直接交出所有权，也clone至少将数据借给别的变量用
    let a = String::from("吃了吗");
    let b = a;
    //println!("{a}");//error，因为此时a已经交出所有权并被释放
    println!("{b}",);

    //但是下面这一段程序则没问题
    let x = 5;
    let y = x;
    println!("x is {x}, y is {y}",);
    //原因是：【x和y并未被转移到heap中】，因为他们都是简单的整型，再stack中就能快速实现。这类操作不分什么深浅拷贝，移动或克隆
    //类似的特性在以下类型的数据上也可实现：
    // • 所有整数类型，比如 u32。
    // • 布尔类型，bool，它的值是 true 和 false。
    // • 所有浮点数类型，比如 f64。
    // • 字符类型，char。
    // • 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String)就没有。

    //当你给函数传递参数的时候，也会遵循类似的原则，在}后面会自动调用一个drop方法来清除无人认领的数据

    let str = String::from("您吃了吗");
    let num = 648;
    print_some(str, num);

    //测试参数传递后的所有权
    //println!("{str}", );//String的所有权会被borrowed，而str不会，integer也不会，原因不在赘述
    println!("{num}",);
}

fn print_some(str: String, num: i32) {
    println!("the string is {str}, and the num is {num}",);
}

//获取String的引用，计算长度后返回
fn calculate_length(str: &String) -> usize {
    //使用引用，str指向的是传入的变量，再由传入的变量指向对应的地址值
    //引用的符号的&，它的逆操作是解引用，符号是*
    str.len()
}

//可变的引用：&mut这样写即可，但同时原来被引用的变量也必须是可变的
//【注意】：原变量的可变引用只能存在一个，如何你先定义了不可变的引用，你就不能在之后定义可变的引用了。这样的限制是为了避免数据竞争
//如果你先定义了一个不可变引用，然后把他们用掉了，你就可以之后再定义一个可变的引用了
fn mutable_reference(str: &mut String) {
    str.push_str(", world");

    //测试可变与不可变引用的彼此作用
    // let mut s1 = String::from("test");
    // let s2 = &s1;
    // let s3 = &mut s1;
    // let s4 = &mut s1;
    // println!("{s1} {s2} {s3} {s4}", );//error: cannot borrow `s1` as immutable because it is also borrowed as mutable

    let mut s1 = String::from("test");
    let s2 = &s1;
    let s3 = &s1;
    println!("{s1} {s2} {s3}", );
    let s4 = &mut s1;
    print!("{s4} \n", );
}

// • 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
// • 引用必须总是有效的。

//接下来我们看看slice：slice 允许你引用集合中一段连续的元素序列，他是引用，故没有所有权

//需求：编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串
//中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
fn first_word(str:&String) -> usize{
    let bytes = str.as_bytes();//将String转为字节数组

    for (i,&item) in bytes.iter().enumerate(){//转化为可迭代的枚举（元组的包装类）并进行遍历
        if item == b' '{//寻找字面值相等的字节
            return i;
        }
    }
    str.len()   
}

// slice的写法可以是[start_index..end_index]，比如slice1=words[0..6]。切片仍然是对于原始String的一个部分引用
// 当省略start_index时，会默认看作0；同理，当省略end_index时，会默认看作最大索引值（也就是最后一位）
// 那么对于上面的first_word，我们只需要将返回值变成str的切片即可，&str[0..i]  &s[..]
// 这些切片本质上是字符串自变量
// 除了字符串切片，我们也可以给数组做切片，写法依旧：let arr = [1,2,3,4,5]; let slice = &arr[1..3];



