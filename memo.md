# 所有权保证在没有垃圾收集器的情况下的内存安全，因为所有的内存都有一个明确的生命周期
- 所有权 Ownership 变量有其所有者
- 变量作用域 变量离开作用域时，值会被丢弃，内存管理会自动释放占用的内存
- 移动 Move 变量作为参数传递时，所有权也会被传递过去，此过程称为Move
- 克隆 Clone 可调用.clone()复制值，允许原始值保留值的所有权，同时获取一个新的副本
- 借用 Borrowing 允许借用值，不拥有他们
 1. 不可变借用 ：允许读取或复制值，不能修改
 2. 可变借用 ： 允许修改值，但是同一时间只能有一个可变借用
- 借用规则 
 1. 要么有一个可变借用 ，或多个不可变借用
 2. 借用必须有效
- 切片Slice 
 1. 对数组的引用，为可变 || 不可变

- 智能指针 ？？？

- 生命周期 Lifetimes

- Drop Trait - 一个值的所有权结束，如果他实现了Drop trait ，Rust会自动调用drop方法来执行清理工作


struct 结构体 要实例化
impl 定义方法
enum 枚举

Vector动态数组
 

- Strings 字符串 || &str --好复杂的  


- Derive 

- 结构Option 一个变量要么有值Some(T)，要么为空None
```   rust
enum Option<T>{
    Some(T),
    None,
}

```
/// 文档注释
cargo doc --open 生成文档

- Result 结果
``` rust
//  True Error
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- HashMap 键值对 

- 闭包  || 

``` rust
fn add(a:i32,b:i32)->i32{
    a+b
}

fn main(){
    let sum = add(1,1);

    let add = |a;i32,b:i32|->{
        a+b
    };

    let add = |a,b| a+b;
    let sum = add(1,1);
}

```

- Map 转换数据

``` rust
 具体看示例 a21
```


- iterator 迭代器

- Range 创建值范围的一种自动方式

- if let 
- while let 
- Modules 模块

- test 测试