use std::collections::HashMap;

pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_sum = i32::MIN;
    let mut left = 0;
    let mut freq_map = HashMap::new();
    let mut window_sum = 0;
    
    for elements in 0..nums.len() as usize {
        window_sum += nums[elements];
        freq_map.entry(nums[elements]).or_insert(0);

        if elements - left == k as usize {
            if freq_map.len() == k as usize {
                max_sum = max_sum.max(window_sum);
            }
        }
    }       
    
    0
}
