fn main() {
    //rust中的函数定义后会被自动提升至作用域的最顶部
    //小知识：函数中定义的形参是parameters，传入的具体值是arguments。虽然一般情况下我们是混着叫的
    //在rust中，每个parameter必须声明可接受的类型，如果你没有声明，那么rust会判断然后替你声明

    //statements_base()
    // control_flow(32);
    // double_loop();
    // while_syntax();
    for_in();
}

fn statements_base() {
    //函数可以被认为是一系列语句和一个可选的结尾表达式构成。
    //语句（statement）：语句可以被认为是执行一些操作但无返回值的指令。
    //表达式（expression）：用于计算并产生一个值。

    // let x = (let y = 6);//error，expected expression, found statement
    //你可能会在别的语言见到x = y =6这样的写法，但是rust并不支持

    let x = {
        let y = 5;
        y + 1 //结尾没有分号，则会将语句产生的值当作表达式返回
    };

    println!("x is {x}"); //y不在定义域内
}

fn control_flow(num: i32) {
    //普通控制流（rust不支持三元表达式，所以你还得写if else）
    if num % 2 > 0 {
        println!("the number is odd");
    } else {
        println!("the number is even");
    }
    //控制流中可以写else if语句进行多重过滤判断，当满足其中一个条件时，会自动退出整个判断遍历。故不必写什么break之类的东西（至少在rust里是这样的）

    //let if语句
    let x = if num % 2 > 0 { "odd" } else { "even" }; //两个条件的返回值必须是同类型的

    // loop语法，会一直执行，直到触发了退出条件，在这里你可能需要使用break或者continue
    let mut y = 0;
    loop {
        y += 1;
        if y > 10 {
            println!("y is bigger than 10 now");
            break;
        }
    }
}

fn double_loop() {
    //这里会接触到一个叫做循环标签（loop label）的概念，不仅适用于双层循环，也适用与多层循环
    let mut count = 0;
    'counting_up: loop {
        //使用'xxx来给外层loop一个标签，标签名为xxx
        println!("count is {count}");

        let mut remaining = 10;

        loop {
            println!("remaining is {remaining}",);

            if remaining == 9 {
                break; //结束内层loop
            }
            if count == 2 {
                break 'counting_up; //结束外层loop
            }
            remaining -= 1;
        }

        count += 1; //外层loop的计数器
    }
    println!("End count = {count}");
}

fn while_syntax() {
    //while是更简便的语法结构
    let mut num = 3;
    while num != 0 {
        println!("number is {num}",);
        num -= 1;
    }

    print!("number is {num} now",);
}

fn for_in() {
    let a = [10, 20, 30, 40, 50];

    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);//读取下标时，不能直接写道{}内
    //     index += 1;
    // }

    //当我们采用上述写法进行数组遍历时，是很不健康的。一方面会容易造成下标溢出的panic，另一方面，每一次loop都要进行一次判断，比较耗性能。
    //此时应当使用for in语法进行遍历
    
    for i in a {
        println!("the value is {i}");
    }

    //来试试另一段
    for num in (1..4).rev(){
        println!("{num}", );
    }
    println!("新年快乐！", );
}
