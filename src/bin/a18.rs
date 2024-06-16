#[derive(Debug)]

struct Adult{
    age:u8,
    name:String,
}

impl Adult{
    fn new(age:u8,name:&str)->Result<Self,&str>{
        if age >= 21 {
            Ok(Self{
                age,
                name:name.to_string(),
            })
        }else{
            Err("Age must be at least 21")
        }
    }
}

fn main(){
    let child = Adult::new(15,"Anita");

    let adult = Adult::new(21,"Sanjay");

    match child{
        Ok(child)=>println!("{},{}",child.age,child.name),
        Err(e)=>println!("{e}"),
    }

    match adult{
        Ok(adult)=>println!("{},{}",adult.age,adult.name),
        Err(e)=>println!("{e}"),
    }
}