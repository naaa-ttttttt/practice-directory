//mod two_pointers;
use crate::intervals::merged_intervals::merge_intervals;
mod intervals;

fn main() {
    println!("{:?}", merge_intervals(vec![
        vec![3, 5], 
        vec![1, 4],
        vec![7, 9],
        vec![6, 8]
        ])
        );
    
    println!("Successful");
}
