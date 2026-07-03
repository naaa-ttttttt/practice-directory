//mod two_pointers;
use crate::intervals::can_attend_meetings::can_attend_meetings;
mod intervals;

fn main() {
    println!("{:?}", can_attend_meetings(vec![
        vec![1, 5], 
        vec![3, 8], 
        vec![6, 9]]
        ));
    
    println!("Successful");
}
