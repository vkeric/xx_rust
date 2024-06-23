# impl为类型提供具体的行为 

1. 实现方法 
 - 为结构体或枚举提供方法

 ```rust 
 struct Rectangle {
    width:u32,
    heiht:u32,
 }

 impl Rectangle {
    fn area(&self)->u32{
           self.width * self.height
    }
 }
 
 ```
2. 实现特征
```rust
trait Animal{
    fn make_sound(&self)
}
struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

```
3. 关联类型
- 在特征中定义的类型,可以在impl中指定具体的类型
```rust
trait HasSize{
    type Size;
}
impl HasSize for Vec<u32>{
    type Size = usize;
}
```
4. 关联函数
```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}
```
5. 泛型实现
```rust 
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
}

```

6. 继承特征实现
当一个类型实现了某个特征，你可以在这个基础上进一步实现其他特征。
```rust
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I am a dog")
    }
}
```

7. 默认实现：
特征可以提供默认方法实现，impl 块中可以覆盖这些默认实现。
```rust 
trait Animal {
    fn make_sound(&self);
    fn animal_type(&self) -> &'static str {
        "Animal"
    }
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }

    // 覆盖默认实现
    fn animal_type(&self) -> &'static str {
        "Dog"
    }
}

```