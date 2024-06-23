# 结构体Struct

# self &self Self 的区别
``` rust
 self 是结构体或枚举实例的默认名称，
 在方法实现中，self指的是访问当前实例的字段和方法
 如果方法需要获取当前实例的可变访问权限，可以使用&mut self 

 &self 方法需要当前实例的不可变引用
   允许访问结构体的字段而不取得所有权，可在不转移所有权的
   情况下调用该方法

Self
Self 是当前结构体或枚举的类型名的别名。
它在 impl 块中用于指代正在实现的类型。
Self 常用于返回当前类型的实例或者在关联函数中作为返回类型。

```


1. 数据聚合 多种类型的数据写在一个结构体中 
```rust

// 结构体
struct Point {
    x:f32,
    y:i32
}

// 实例化
let point = Point { x:10.0,y:10 }
```

2. 数据封装
```rust
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self)->u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 10, height: 20 };
println!("Area: {}", rect.area());

```
3. 模式匹配
```rust
enum Color{
    Red,
    Blue,
    Green,
}

Struct Circle{
    radius:f32,
    color:Color,
}

// 实例化结构体
let circle = Circle {
    radius:2.0,
    color:Color::Red,
}

// 对实例化结构体进行模式匹配
match circle {
    // 颜色是红色时=>返回值 // 颜色是蓝色时=>返回值//..剩余参数

    Circle {color:Color::Red,..}=>println!("Red circle"),
    Circle {color:Color::Blue,..}=>println!("Blue circle"),
    _ => (),
}



```
4. 实现特征
```rust

trait Animal{
    fn make_sound(&self)
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self){
        println!("woof!");
    }
}

// 实例化 dog有
let dog = Dog;
dog.make_sound();


// 为 Dog 实现 Animal的 特征
```

5. 泛型编程
```rust 
// 定义泛型结构体:
struct Stack<T>{
    items:Vec<T>
}

// 为泛型结构体实现方法:
impl<T>Stack<T>{
    fn new() -> Self{
        Stack {items:Vec::new()}
    }
}

let int_stack = Stack::new();
// 声明变量 的类型是 调用关联函数
let string_stack: Stack<string> = Stack::new();
```

6. 数据传输
```rust
struct User {
    id: u32,
    username: String,
    email: String,
}

// 假设这是从 API 接收到的数据
let user = User {
    id: 1,
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
};
```
7. 测试和模拟
```rust
struct Account {
    balance: f32,
}

// 测试用例中模拟的账户数据
let test_account = Account { balance: 1000.0 };
```

8. 构建大型项目
```rust 
// 假设这是大型项目中的一个模块
mod game {
    struct Player {
        name: String,
        health: u32,
        mana: u32,
    }

    impl Player {
        fn new(name: String) -> Self {
            Player {
                name,
                health: 100,
                mana: 50,
            }
        }
    }
}

// 使用模块中定义的结构体
let player = game::Player::new("Alice".to_string());
```

9. 智能指针??? 不懂什么意思 -太复杂以后看24/6/23
```rust 
// 这段 Rust 代码演示了如何使用 Rc（引用计数智能指针）来创建和管理一个简单的双向链表。以下是代码的逐行注释：

// 引入标准库中与 Rc 相关的模块。
use std::rc::Rc;

// 定义结构体Node -value值和 next 
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

let node1 = Rc::new(Node { value: 1, next: None });
let node2 = Rc::new(Node {
    value: 2,
    next: Some(node1.clone()),
});



```