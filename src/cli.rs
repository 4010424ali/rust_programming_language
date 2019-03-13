use std::env; 

pub fn run(){
    let args: Vec<String> = env::args().collect(); 
    let command = args[1].clone();
    let name =  "Rana Usama Ali";

    if command == "hello" {
        println!("hi {}, how you are you", name);
    }

    // println!("Args : {:?}", command);
}