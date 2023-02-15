//有过面向对象语言和含有类的语言的学习经历更容易理解本章
// 结构体的每一项成员都可以是不同的类型（这一点和元组有些像），但这些成员必须要有名字。除此之外，整个结构体也得有一个属于他自己的名字
// 使用struct关键字来定义一个结构体，定义一个初始结构体必须要把各个成员的类型也规定上
struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}
// 结构体实例化的过程，也是给这些成员赋予具体的值的过程（这些值必须满足初始结构体中给的类型）

fn main() {
    //实例化一个结构体，语法是: struct_name {}
    let mut user1 = User{
        active:true,
        username:String::from("赵家辉"),
        email:String::from("2624604133@qq.com"),
        sign_in_count:1
    };

    // 可以通过面向对象的点语法来快速访问和修改实例化的结构体内部的成员(但是你首先得给实例化的结构体权限：加个mut关键字)
    user1.username = String::from("这货不是赵家辉");
    let s = user1.username;
    println!("{s}", );

    let mut user2 = create_new_user(String::from("18093166486@xxx.com") , String::from("我是马牛逼"));
    let s = user2.username;
    println!("{s}", );

    // 如果我们的一个新实例和旧实例很像，我们可以采用结构体更新语法，在旧结构体的基础上却实例化新的结构体。语法类似于扩展字符串
    let user3 = User {
        username:String::from("moos"),
        ..user1
    };
    let s = user3.username;
    println!("{s}", );

    //元组结构体
    let black = RBG(0,0,0);
}

// 你也构建一个函数来返回一个结构体（规定返回一个结构体）
fn create_new_user(email:String , username:String) -> User{
    User{
        email,//k v一致时的简写形式
        username,
        active:true,//结构体成员设置默认值
        sign_in_count:1
    }
}

// 另一种元组结构体，在声明时用小括号，同时只用声明类型即可。有了元组结构体，我们可以使得多个有连接的数据的彼此间的关系展示出来
struct RBG(u32,u32,u32);

// 第三组类单元结构体（unit-like），我们暂时还用不到，他是类似与()的作用，他只有一个struct的声明语句和结构体名称