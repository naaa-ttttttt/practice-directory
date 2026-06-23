pub fn max_points(cards: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut left = 0;
    let mut right = cards.len() - 1;
    let mut window_sum = 0;
    let mut max_sum = 0;
    
    for i in 0..k {
        window_sum += cards[i];
    }


    0
}
