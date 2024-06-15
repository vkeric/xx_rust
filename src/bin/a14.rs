struct Person{
    name:String,
    fav_color:String,
    age:i32
}

fn print(data:&str){
    println!("{:?}",&data)
}

fn main(){
    let people = vec![
        Person{
            name:String::from("1"),
            fav_color:String::from("1"),
            age:1,
        },
        Person{
            name:String::from("2"),
            fav_color:String::from("2"),
            age:2,
        },
        Person{
            name:String::from("3"),
            fav_color:String::from("3"),
            age:3,
        }        
    ];

    for person in people {
        if person.age<=10{
            print(&person.name);
            print(&person.fav_color);
        }
    }
}