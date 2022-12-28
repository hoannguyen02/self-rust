* Get what user type in console
```
use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn main() {
    let mut allow_inut = vec![];
    let mut times_input = 0;
    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                allow_inut.push(words);
                times_input += 1;
            }
            Err(e) => println!("Error: {:?}", e)
        }
    }

    for input in allow_inut {
        println!("Input {:?}", input)
    }
}

```

```
use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}


impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
                "off" => Some(PowerState::Off),
                "sleep" => Some(PowerState::Sleep),
                "reboot" => Some(PowerState::Reboot),
                "shutdown" => Some(PowerState::Shutdown),
                "hibernate" => Some(PowerState::Hibernate),
                _ => None
        }
    }
}

fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("turning off") ,
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shuting down"),
        Hibernate => println!("hibernating")
    }
}

fn main() {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("Invalid power state")
        }
    }
}
```