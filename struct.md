
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

