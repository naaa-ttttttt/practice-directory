//mod two_pointers;
use crate::stacks::decode_string::decode_strings;
mod stacks;

fn main() {
    println!("{:?}", decode_strings(String::from("3[a2[c]]")));
    
    println!("Successful");
}
