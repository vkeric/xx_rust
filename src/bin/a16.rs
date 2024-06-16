 struct Student {
    name:String,
    locker:Option<i32>,
 }

 fn main(){
    let mary = Student {
        name:"Mary".to_owned(),
        locker:Some(3),
    };

    println!("{:?}",mary.name);

    match mary.locker{
        Some(num)=> println!("{:?}",num),
        None => println!("no locker assigned"),
    }
 }