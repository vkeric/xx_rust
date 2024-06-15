// Vectors






fn main(){
    let my_numbers = vec![10,20,30,40];

    for num in &my_numbers {
        match num {
            30 =>println!("30"),
            _ =>println!("{:?}",anum),
        }
    }
    println!("{:?}", my_numbers.len());
}