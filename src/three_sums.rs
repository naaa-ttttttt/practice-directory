fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = Vec::new();
    let len = nums.len();

    for i in 0..len{
        if i > 0 && nums[i] == nums[i - 1] {
            continue; 
        }

        let mut left = i + 1; //this is the element after the first element.
        let mut right = len - 1; // this is the last element on the list.

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum < 0 {
                left += 1;
            } else if sum > 0 {
                right -= 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
            }
        
            while left < right && nums[left] == nums[left + 1] {
                left += 1;
            }

            while left < right && nums[right] == nums[right - 1] {
                right -= 1;
            }

            left += 1;
            right -= 1;
        }
    }
    result

}
