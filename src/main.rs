//mod two_pointers;
use crate::sliding_window::char_replacement::character_replacement;
mod sliding_window;

fn main() {
    println!("{:?}", character_replacement(String::from("eghghgg"), 2));
    
    println!("Successful");
}
