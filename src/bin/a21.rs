#[derive(Debug)] //属性宏

// 结构体
struct User {
    user_id:i32,
    name:String,
}

fn find_user(name:&str)->Option<i32>{
    let name = name.to_lowercase();
    match name.as_str(){
        "sam"=>Some(1),
        "matt"=>Some(5),
        "katie"=>Some(9),
        _=>None,
    }
}

fn main(){
    let user_name = "sam";
    // 实例化
    let user = find_user(user_name).map(|user_id| User{
        user_id,
        name:user_name.to_owned(),
    });
    match user {
        Some(user)=>println!("{:?}",user),
        None=>println!("user not found"),
    }
}