pub fn max_of_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut left = 0;
    let mut window_sum = 0;
    let mut max_sum = i32::MIN;
    let len = nums.len();

    for right_ptr in 0..len {
        window_sum += nums[right_ptr];

        if (right_ptr - left) + 1 == k {
            max_sum = max_sum.max(window_sum);
            
            window_sum -= nums[left];

            left += 1;
        }
    }
    max_sum
    
}
