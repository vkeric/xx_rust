
// 枚举
enum Color {
    Red,
    Yellow,
    Blue,
}

fn print_color(my_color:Color){
    // 根据传入参数判断
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue"),
    }
}


fn main(){
    print_color(Color::Blue);
}