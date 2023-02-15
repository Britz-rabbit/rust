fn main() {
    // test_shadow();
    // test_shadow_let();
    data_type();
}

//shadow
fn test_shadow() {
    //隐藏shaodw可以使用let关键字或者直接使用原变量名来重新赋值
    let mut x = 5;
    println!("The value of x is: {x}");
    x = x + 1;
    {
        let x = x * 2; //let不光会隐藏之前的值，它实际上创建了一个新的值，但在作用域结束后就被释放了
        println!("The value of x in the inner scope is: {x}");
    }
    x = x + 1;
    println!("The value of x is: {x}");
}

//shaodw and let
fn test_shadow_let() {
    //当我们要对原本的变量进行处理的时候，如果要借助隐藏，则【必须】使用let关键字重新声明，声明后的变量仍然是默认不可变的
    let mut spaces = "   ";
    let spaces = spaces.len();//即使你声明了mut关键字，也必须使用let来重新声明，因为mut也无法容忍你改变了数据类型
    println!("you print {spaces} spaces");
}

fn data_type(){
    //rust中的数据类型大体可分为标量和复合两大类
    //标量（scalar）:整数（sign或unsign），浮点数(float,都是带符号的，默认f64)，布尔值(bool)，字符类型(&str或String，使用双引号。单引号的是char的字母类型)
    //复合（compound）：元组（tuple）和数组（array）。元组的特点是一旦声明，长度不改变

    //元组可以指定对应位置的数据类型
    let tup:(i32 , i32 ,f64) = (504 , 233 , 6.04);
    let (x , y , z) = tup;//解构
    println!("the value of y is {y}");
    let i = tup.2;//使用.直接访问元组索引对应的值
    println!("{i}");
    //【注意】当一个表达式没有返回值时，它会隐式地返回一个()，被称为单元，表示空值或空的返回类型

    //数组要求其内的数据类型必须一样，且长度不可变
    let arr:[i32;5] = [1, 2, 3, 4, 5];
    let arr2 = [3;5];//简便写法，数组内包含5个3
    println!("{:?}",arr2);
    let a = arr[3];
    //数组通过[]来访问索引的下标值
    print!("{a}", );
    //向量（vector）与数组类似，但是其长度可变，因此比数组更加灵活
    
}
