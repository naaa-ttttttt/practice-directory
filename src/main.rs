//mod two_pointers;
use crate::sliding_window::max_sum::max_of_subarray;
mod sliding_window;

fn main() {
    println!("{:?}", max_of_subarray(vec![2, 1, 5, 1, 3, 2], 3));
    
    println!("Successful");
}
