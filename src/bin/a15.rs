// 15

// 枚举一下
enum Ticket {
    Backstage(f64,String),
    Standard(f64),
    Vip(f64,String),
}

// 主函数
//  :: 访问 
fn main(){
    let tickets = vec![
        Ticket:: Backstage(50.0,"Billy".to_owned()),
        Ticket:: Standard(15.0),
        Ticket:: Vip(30.0,"Amy".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price,holder)=> {
                println!("holder{:?},price{:?}",holder,price)
            }
            Ticket::Standard(price)=>println!("price{:?}",price),
            Ticket::Vip(price,holder)=>{
                println!("holder{:?},price{:?}",price,holder)
            }
        }
    }
}