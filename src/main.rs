//mod two_pointers;
use crate::stacks::longest_v_substring::longest_v_substring;
mod stacks;

fn main() {
    println!("{:?}", longest_v_substring(String::from("((()()())")));
    
    println!("Successful");
}
