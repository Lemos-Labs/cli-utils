use std::io;
fn main() {
    println!("Command Line utilities in Rust made with <3 by lemosep");
    
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("error: couldn`t read line");

        let trimmed = input.trim();

        if trimmed == "exit" {
            break;
        };

        match trimmed {
            "ls" => println!("Off to run da LS command!"),
            _ => println!("no command available, nigga")
        }

    }

}
