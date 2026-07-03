//mod two_pointers;
use crate::intervals::insert_intervals::insert;
mod intervals;

fn main() {
    println!("{:?}", insert(vec![
        vec![1, 2], 
        vec![3, 5], 
        vec![6, 9],
        vec![8, 10]]
        , vec![3, 5])
        );
    
    println!("Successful");
}
