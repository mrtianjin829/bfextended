mod bfextended;
use crate::bfextended::BFExtended;
use std::{env,fs,io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [FILEPATH]\n if filepath is - then it will read a single line from stdin",args[0]);
        std::process::exit(2); 
    };
    let filepath = &args[1];
    let mut context = BFExtended::new();
    if filepath == "-" {
        let mut code = String::new();
        io::stdin().read_line(&mut code).expect("Failed to read from stdin");         
        context.run(&code);
        return;
    }

    let code = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(bob) => {
            println!("Failed to read the file: {}",bob);
            std::process::exit(2);
        }
    };


    context.run(&code);
}
