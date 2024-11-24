use crate::append_function::append::appending;
use std::io;

pub fn main() {
    println!("=======================================================================");
    println!("Enter a string of characters/words and see them converted to pig latin!");
    println!("For each word/character you want converted into a string of pig latin, ");
    println!("enter a space between. Press 'enter' once done. Keep the list of chara-");
    println!("cters on one line.");
    println!("=======================================================================");
    loop {
        let mut user_strings = String::new();
        io::stdin().read_line(&mut user_strings).expect("Failed to read line");
        let formatted = user_strings.trim().to_lowercase();
        appending(formatted);
        break;
    }
}