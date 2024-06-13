// 结构体

// 枚举
enum Flavor{
    Sparkling,
    Sweet,
    Fruity,
}

// 结构体
struct Drink{
    flavor:Flavor,
    fluid_oz:f64,
}


//  函数接受 drink 匹配传入进来的flavor
fn print_drink(drink: Drink){
    match drink.flavor{
        Flavor::Sparkling =>println!("sparkling"),
        Flavor::Sweet =>println!("Sweet"),
        Flavor::Fruity =>println!("Fruity"),
    }
    println!("oz:{:?}",drink.fluid_oz);
}


fn main(){
    // 实例化结构体
    let Sweet = Drink{
        flavor:Flavor::Sweet,
        fluid_oz:6.0,
    };
    print_drink(Sweet);

    let fruity = Drink{
        flavor:Flavor::Fruity,
        fluid_oz:10.0
    };
    print_drink(fruity);
}