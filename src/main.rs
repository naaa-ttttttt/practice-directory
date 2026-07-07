//mod two_pointers;
use crate::intervals::insert_intervals::insert;
mod intervals;

fn main() {
    println!("{:?}", insert(vec![
        vec![1, 3], 
        vec![6, 9],
        ], vec![2, 5])
        );
    
    println!("Successful");
}
