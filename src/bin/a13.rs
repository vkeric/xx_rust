// Vectors






fn main(){

    let my_number = vec![10,20,30,40];
    for num in &my_number{
        match num {
            30=>println!("30"),
            _=>println!("{:?},"num),
        }
    }
    println("{:?}", my_number.len());
}