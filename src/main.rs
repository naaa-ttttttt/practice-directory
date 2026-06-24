//mod two_pointers;
use crate::sliding_window::max_dist::max_sum;
mod sliding_window;

fn main() {
    println!("{:?}", max_sum(vec![5, 1, 8, 3, 2, 9, 4], 3));
    
    println!("Successful");
}
