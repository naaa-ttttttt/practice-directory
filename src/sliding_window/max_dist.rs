use std::collections::HashMap;

pub fn max_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut max_sum = i64::MIN;
    let mut left = 0;
    let mut freq_map = HashMap::new();
    let mut window_sum = 0i64;
    
    for elements in 0..nums.len() as usize {
        window_sum += nums[elements] as i64;
        *freq_map.entry(nums[elements]).or_insert(0) += 1;

        if elements - left + 1 == k as usize {
            if freq_map.len() == k as usize {
                max_sum = max_sum.max(window_sum);
            }
            // so the first condition checks if window size is equal to the target,
            // second condition checks if every element in the hashmap is different,
            // if it satisfies both conditions, max sum is updated, else, the code 
            // below removes the element at the left part of the sliding window, it 
            // then gets the key value from the hashmap and so since it has been reduced 
            // from the sliding window, its count on the hashmap is been reduced, and if
            // its count is equal to 0, then we remove the element off the hashmap.

            window_sum -= nums[left] as i64;
            *freq_map.get_mut(&nums[left]).unwrap() -= 1;
            if freq_map[&nums[left]] == 0 {
                freq_map.remove(&nums[left]);
            }

            left += 1;
        }
    }       

    if max_sum == i64::MIN {
       0
    } else {
        max_sum
    }
}
