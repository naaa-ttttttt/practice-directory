//mod two_pointers;
use crate::intervals::non_overlapping::non_overlapping_intervals;
mod intervals;

fn main() {
    println!("{:?}", non_overlapping_intervals(vec![
        vec![1, 3], 
        vec![5, 8],
        vec![4, 10],
        vec![11, 13]
        ])
        );
    
    println!("Successful");
}
