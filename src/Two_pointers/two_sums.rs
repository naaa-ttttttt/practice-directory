fn two_sum(nums: &[i32], target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let current_sum = nums[left] + nums[right];
        if current_sum == target {
            return true;
        }
        if current_sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    
    false
}

