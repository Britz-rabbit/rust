// • 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
// • Crates ：一个模块的树形结构，它形成了库或二进制项目。
// • 模块（Modules）和 use：允许你控制作用域和路径的私有性。
// • 路径（path）：一个命名例如结构体、函数或模块等项的方式

// crate是最小的代码单位，包括二进制项和库两种形式，一个carte必须要包括一个main.rs，是一段可运行的程序。通常在做项目的时候，这些carte都是以库的形式存在的
// packages是包含多个crate的一系列功能
// • 模块中的代码路径: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一
// 个 crate 内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模
// 块下的Asparagus类型可以在crate:: garden::vegetables::Asparagus被找到。

// • 私有 vs 公用: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使
// 用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。

// • use 关键字: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任
// 何可以引用crate:: garden::vegetables::Asparagus的作用域, 你可以通过use crate::garden::vegetables::Aspara建一个快捷方式，
// 然后你就可以在作用域中只写Asparagus来使用该类型。

fn main() {
    println!("Hello, world!");
}
