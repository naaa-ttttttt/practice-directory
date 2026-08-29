//mod two_pointers;
use crate::heap::k_closest::k_closest;
mod heap;

fn main() {
    println!("{:?}", k_closest(1, 3, vec![-1, 0, 1, 4, 6]));
    
    println!("Successful");
}
