// Fábio Gandini - 2022
// Rust user input example.

//importing IO library:
use std::io;


fn main() {
	println!("> Please type something: ");
    
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            println!("You typed: {}", user_input);
        },
        Err(e) => {
            println!("Algo deu errado: {}", e)
        }
    }
}
