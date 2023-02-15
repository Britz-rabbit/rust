// 来看看struct的一些应用场景

// 为函数的参数设置有意义的输入参数
#[derive(Debug)] //为结构体定义前加上外部属性，使得我们能打印出来
struct Rect {
    width: u32,
    height: u32,
}

//与结构体对应的是方法，酒行函数一样，他们也有参数和返回值语句。这些方法的第一个参数是self，代表结构体实例
// 这些方法通过impl关键字来加到对应的结构体内
impl Rect{
    //出于和之前相同的原因，我们这里采用结构体的引用
    fn rect_area(&self) -> u32{
        self.width * self.height
    }
    //对一个结构体可以同时引入多个方法
    fn width(&self)->u32{
        self.width
    }
}


fn main() {
    let mut rect1 = Rect {
        width: 30,
        height: 50,
    };
    
    println!("the area of rect is {} square pixels", {area(& mut rect1)});
    print_width(&rect1);//授权之后，rect1.width被改了

    // 上面的写法只是为了看到结构体内部的某项属性，当我们试图直接打印结构体实例的时候，会出现错误，为此我们要做一些小小的改变
    // 首先我们要加入外部属性，之后使用调试格式来打印结构体实例
    // println!("{:?}", rect1);
    // 你也可以换成另一种调试的打印风格,在: 和 ? 之间加一个#
    print!("{:#?}", rect1);

    // 在加入Debug的trait后可以使用dbg宏来打印结构体，dbg就是debug的意思，所以打印出来的结果会输出到debug对应的消息队列
    // dbg!(&rec) //好吧，我找不到debug对应的队列，我觉得我也不太需要这个

    // 使用结构体内的方法来计算面积。直接调用rect1内部的方法即可。这也是为什么被称为方法的原因--它是在结构体内部调用的
    println!("\n the area of rect is {} square pixels", rect1.rect_area());
    //当你使用rect1.width()的时候，你调用的是自定义的方法，当你使用rect1.width来读取值的时候，其实是调用了getters方法
    println!("\n the width of rect is {} pixels", rect1.width());

    //这些类方法的使用语法是： 结构体::类方法名 ，如下：
    let square = Rect::square(32);
    println!("\n {:#?}", square);
}

//我们获取的是结构体的引用，这样结构体实例就不会被释放掉
// 也可以就给这些方法加入其他参数，如can_hold(&self,&Rect)，通过rect1.can_hold(&rect2)这样调用即可
fn area(rect:& mut Rect) ->u32{
    rect.width -= 1;
    rect.width * rect.height//对于引用的结构体的字段的使用，不会影响字段本身的使用，但是你更改的话需要授权（实例和引用双重授权）
}

fn print_width(rect:&Rect){
    print!("the width of rect is {} px \n",rect.width );
}

// 除去常规的方法，我们也可以不以self为第一参数，而是以一些别的参数。但是这样的类方法也可以被应用于结构体，如下
impl Rect{
    //此处大写的Self代表被应用的结构体，此处代表Rect
    fn square(size:u32)->Self{
        Self{
            width:size,
            height:size
        }
    }
}