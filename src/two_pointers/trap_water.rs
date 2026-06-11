fn trapping_water(heights: Vec<i32>) -> i32 {
    let mut count = 0;
    let length = heights.len();
    let mut left = 0;
    let mut right = length - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    
    while left < right {
        left_max = std::cmp::max(left_max, heights[left]);
        right_max = std::cmp::max(right_max, heights[right]);

        if left_max < right_max {
            let water = left_max - heights[left];
            count += water;
            left += 1;
        } else {
            let water = right_max - heights[right];
            count += water;
            right -= 1;
        }
    }
    count 
}
