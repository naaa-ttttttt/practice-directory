//mod two_pointers;
use crate::stacks::valid_parenthesis::valid_parenthesis;
mod stacks;

fn main() {
    println!("{:?}", valid_parenthesis("(){({})}"));
    
    println!("Successful");
}
