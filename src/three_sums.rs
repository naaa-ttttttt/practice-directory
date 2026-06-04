fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = Vec::new();
    let len = nums.len();

    for i in 0..len{
        if i > 0 && nums[i] == nums[i - 1] {
            i += 1;
        }

        let left = i + 1; //this is the element after the first element.
        let right = len - 1; // this is the last element on the list.

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum < 0 {
                left += 1;
            } else if sum > 0 {
                right -= 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
            }

        }
    }
}
