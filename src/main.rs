//mod two_pointers;
use crate::intervals::employee_free_time::employee_free_time;
mod intervals;

fn main() {
    println!("{:?}", employee_free_time(vec![
        vec![
        vec![3, 5], vec![1, 4]
        ],
        vec![
        vec![7, 9],
        vec![6, 8],
        ]])
        );
    
    println!("Successful");
}
