


fn print_message(gt_100:bool){

    match gt_100 {
        true =>println!("its big"),
        false =>println!("its small"),
    }
}

fn main(){
    let value = 100;
    let is_gt_100 = value>100;
    print_message(is_gt_100);
// 提交
}