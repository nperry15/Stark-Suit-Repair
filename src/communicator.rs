#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        let s;
        match self{
            Command::Power(b, percent) => (if *b {s = format!("Power increased by {}%", percent)} else {s = format!("Power decreased by {}%", percent)}),
            Command::Missiles(b,n) => (if *b {s = format!("Missiles increased by {}", n)} else {s = format!("Missiles decreased by {}", n)}),
            Command::Shield(b) => (if *b {s = "Shield turned on".to_string()} else {s = "Shield turned off".to_string()}),
            Command::Try => s = "Call attempt failed".to_string(),
            Command::Invalid => s = "Not a command".to_string(),
        }

        return s;
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    let mut iter = s.split(' ');
    match iter.next(){
        Some("power") => {let b;
            match iter.next(){
              Some("inc") => {b = true;},
              Some("dec") => {b = false;},
              _ =>  {return Command::Invalid},
            }
            match iter.next(){
              Some(num) => {match iter.next(){
                            None => {let percent : i32 = num.parse().unwrap();
                                      Command::Power(b, percent)},
                            _ =>  {return Command::Invalid;},
                          }},
              _ =>  {return Command::Invalid;},
            }
    }
      Some("fire") => {let b = false;
                  let num : i32 = iter.next().unwrap().parse().unwrap();
                  match iter.next(){
                    Some("missiles") => return Command::Missiles(b, num),
                    _ => return Command::Invalid,}
      }
      Some("add") => {let b = true;
            let num : i32 = iter.next().unwrap().parse().unwrap();
            match iter.next(){
              Some("missiles") => return Command::Missiles(b, num),
              _ => return Command::Invalid,}
      }
      Some("shield") => {let b;
                  match iter.next(){
                    Some("on") => {b = true;},
                    Some("off") => {b = false;},
                    _ =>  {return Command::Invalid},
                  }
                  match iter.next(){
                    None => {return Command::Shield(b);},
                    _ => {return Command::Invalid;}
                  }
      }
      Some("try") => {
                  match iter.next(){
                    Some("calling") => {match iter.next(){
                                        Some("Miss") => {match iter.next(){
                                                          Some("Potts") => {match iter.next(){
                                                            None => {return Command::Try;},
                                                            _ => {return Command::Invalid;}
                                                          }},
                                                          _ => {return Command::Invalid;}
                                                        }},
                                        _ => {return Command::Invalid;}
                                      }},
                    _ =>  {return Command::Invalid},
                  }
      }
      _ => {return Command::Invalid}}
}
