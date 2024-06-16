/**
在Rust中，HashMap 是 std::collections 模块中的一个数据结构，
用于存储键值对集合。HashMap 提供了快速的数据插入和检索功能，
并且键值对中的键必须是唯一的。HashMap 不保证元素的顺序。
*/
use std::collections::HashMap;


fn main(){
    let mut stock = HashMap::new();
    stock.insert("Chair",5);
    stock.insert("Bed",3);
    stock.insert("Table",2);
    stock.insert("Couch",0);

    let mut total_stock = 0;

    for (item,qty) in stock.iter(){
        total_stock = total_stock + qty;
        let stock_count = if qty == &0{
            "out of stock".to_owned()
        }else{
            format!("{:?}",qty)
        };
        println!("{:?},{:?}",item,stock_count);
    }
    println!("{:?}",total_stock);
}