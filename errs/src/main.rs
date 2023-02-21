// Rust 将错误分为两大类：可恢复的（recoverable）和 不可恢复的（unrecoverable）错误。对于一个可
// 恢复的错误，比如文件未找到的错误，我们很可能只想向用户报告问题并重试操作。不可恢复的错误总
// 是 bug 出现的征兆，比如试图访问一个超过数组末端的位置，因此我们要立即停止程序。

// panic!是一个可以展开并清理栈数据并退出程序的宏，用于处理一些可能的bug
// 展开（unwinding）：rust会回溯栈并清理遇到的每个函数的数据
// 终止（abort）：不清理数据，直接退出。rust的默认操作是展开，如果你需要设置rust来执行abort，需要在Cargo.toml 的 [profile ] 部分增加 panic = 'abort'

/*
fn main() {
    // panic!("crash and burn");//主动调用panic！宏

    // let v =vec![1,2,3];
    // v[99];//thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', main.rs:15:5

    //在c语言中，你这样做会读取到别的数据的值，即你都到的这部分内存空间并不属于这个数据结构。该现象被称为缓冲区溢出，是一种非常危险的行为，可能被用来获取秘密数据

    //尝试阅读backtrace来找一下问题所在，不过我们首先要把 RUST_BACKTRACE设置为非0值
}
*/

// 对于可恢复的错误，我们一般是panic!配合Result枚举类处理
// enum Result<S,F> {//T代表成功的返回值，E是错误的返回值，里面是二者的回调，箭头之后是回调的具体处理（你也可以不写箭头表示你不愿意处理它）
//     Ok(S),//这里的泛型参数叫什么都无所谓，我就叫S和F了，代表success和fail
//     Err(F)
// }//这只是一个例子，当我们需要它的时候我们会自己写对应实例的回调的

/*
fn main(){
    use std::fs::File;
    // 打开文件的操作的返回值是一个result类型的枚举类
    // let f:u32 = File::open("test.txt");//error: mismatched types // label: expected `u32`, found enum `std::result::Result`
    let f = File::open("test.txt");//当文件打开成功的话，f将会是一个被Ok调用的实例，失败则是被Err调用

    // 在获取实力后我们通过match语句来处理
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("error in opening file : {:?}", error), //thread 'main' panicked at 'error in opening file : Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }', main.rs:38:23
    };//这是一个赋值语句，我们要以;来结尾

    print!("test", );//当上面的match语句执行后，执行了panic!宏并退出程序，所以这段打印是不会执行的
}
*/

//有时候，我们不想直接退出，而是尝试着去处理

/*
fn main(){
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("test.txt");

    let f = match f {
        Ok(res) => res,
        Err(err) => match err.kind() {//调用err.kind方法做进一步判断处理，此时的返回值不再是一个简单的result枚举类
            ErrorKind::NotFound => match File::create("test.txt"){//调用File::create()方法并根据结果进行第三重判断，其返回值是一个result枚举类
                Ok(res) => res,
                Err(err) => panic!("error in creating a file : {}",err)
            },
            other => panic!("problem in opeing a file {}", other)
        }
    };
}
*/

//上述逻辑的另一种实现是借助闭包和unwarp_or_else（为了避免回调地狱，你懂的）：

/*
fn main() {
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("test.txt").unwrap_or_else(|error| {
        //这个函数是一种只需要处理error值的情况时使用的，省略了成功的默认值
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("problem creating the file file : {:?}", error);
            })
        } else {
            panic!("problem opening the file : {:?}", error);
        }
    });
}
*/

// 除去成功的默认值的缩写，也可以有失败时的panic的缩写写法
// use std::fs::File;

// fn main(){
//     // let f = File::open("test.txt").unwrap();//thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }', src\main.rs:91:36
//     let f = File::open("test.txt").expect("这是一个错误");//相比于unwrap，这个方法会允许你自定义一个错误信息的打印
//     print!("test", );//只是简化了panic的调用，但是rust依然会退出程序，test不会被打印
// }

// 错误传播：指除了编写者，让函数的调用者也知晓调用的可能风险，并决定如何处理。如下：
use std::fs::File;
use std::io::{self, Read};

/*
fn read_username_from_file() -> Result<String, io::Error> {//跑通的话，就返回一个string，否则返回一个error类型的数据结构
    let f = File::open("hello.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e)//若没有文件，则返回一个err结构
    };

     let mut s = String::new();
     match f.read_to_string(&mut s){//若有文件，则读取字
         Ok(_) => Ok(s),
         Err(e) => Err(e)//若读取失败，则返回另一个err结构
     }

 }//到此为止，我们及返回了错误的信息，又没用使程序panic掉，我们用io里的一个error数据结构去代替了读写产生的panic
*/

// 另一个数据传播的简写是 ? 运算符
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;//? 的意思是如果有，则赋值为成功，否则赋值为失败并返回整个失败的数据结构
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;//同上
//     Ok(s)
// }

//更过分一点的方式是直接链式调用
// fn read_username_from_file() -> Result<String, io::Error> {
// let mut s = String::new();
// File::open("hello.txt")?.read_to_string(&mut s)?;// 23333
// Ok(s)
// }

// use std::fs;
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// fn main() {
//     read_username_from_file();
// }

// 使用 ? 的前提是什么？函数的返回值必须与 ? 的作用值相兼容:目前包括Result和Option
// fn main(){
//        let f = File::open("hello.txt")?;
// //     error: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
// //     label: cannot use the `?` operator in a function that returns `()`
// }

// // 尝试对返回值为option 的情况来使用 ?
// fn last_char_of_first_line(text:&str) -> Option<char> {
//     // 使用lines作为每一行的迭代器，通过next获取行的内容（其返回值是一个Option类，若有值，则返回Some(char),若为空，则返回None）。
//     // 若返回值为Some(char)，则将Some(char)转化为chart，并获得最后一次迭代的结果
//     // 注意：你可以在返回 Result 的函数中对 Result 使用 ? 运算符，可以在返回 Option 的函数中对 Option 使用 ? 运算符，但是不可以混合搭配。
//     // 因为 ? 并没有将option转化为result，或者将result转化为option的效果。你应该使用ok()或者ok_or()来达到效果
//     text.lines().next()?.chars().last()
// }

// fn main(){
//     assert_eq!(//assert_eq!是一个宏，用做评估两个表达式是否相等，相比于assert!，这个宏提供了更多错误信息以供调试
//         last_char_of_first_line("hello little shit,\nhow are you doing today?"),
//         Some('d')
//     );
//     assert_eq!(last_char_of_first_line(" "), None);
//     assert_eq!(last_char_of_first_line("\nhi"), None);
// }

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
