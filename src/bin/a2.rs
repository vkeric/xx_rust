// 两数之和

// 函数接受i32的参数->返回i32的结果
fn sum(a:i32,b:i32)-> i32{
    a+b
}

// 打印结果
fn display_result(result:i32){
    println!("{:?}",result); // 打印宏
}


// main 主函数
fn main(){
    let result = sum(1,2);
    display_result(result);
}

