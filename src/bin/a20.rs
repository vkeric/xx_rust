use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

// 方法
impl PowerState {
    fn new(state: &str)->Option<PowerState>{
        let state:String = state.trim().to_lowercase();
        match state.as_str() {
            "Off"=>Some(PowerState::Off),
            "Sleep"=>Some(PowerState::Sleep),
            "Reboot"=>Some(PowerState::Reboot),
            "Shutdown"=>Some(PowerState::Shutdown),
            "Hibernate"=>Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_action(state:PowerState){
    use PowerState::*;
    match state {
        Off => println!("turning off"),
        Sleep => println!("Sleep"),
        Reboot => println!(" Reboot"),
        Shutdown => println!(" Shutdown"),
        Hibernate => println!("Hibernate"),
    }
}

fn main(){
    let mut buffer = String::new();
   let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok(){
        match PowerState::new(&buffer){
            Some(state)=>print_power_action(state),
            None =>println!("invalid power state"),
        }
    }else{
        println!("error reading input ");
    }
}