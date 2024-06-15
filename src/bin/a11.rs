// 所有权——你知道我的故事？


// 定义结构体
struct GroceryItem{
    quantity:i32,
    id:i32,
}

fn display_quantity(item: &GroceryItem){
    println!("{:?}",item.quantity);
}

fn display_id(item: &GroceryItem){
    println!("{:?}",item.id);
}

fn main(){
    // 实例化结构体
    let my_item = GroceryItem{
        quantity:3,
        id:99,
    };
    display_quantity(&my_item);
    display_id(&my_item);
}