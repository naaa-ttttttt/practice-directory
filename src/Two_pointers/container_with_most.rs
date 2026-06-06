pub struct Solution;

impl Solution {
    pub fn max_area(array: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut current_max = 0;

        while left < right {
            let width = (right - left) as i32;
            let height = std::cmp::min(array[left], array[right]); //height is the shorter value in the array, so height equal to min value.
            let area = width * height;

            current_max = std::cmp::max(current_max, area);

            if array[left] < array[right] {
                left += 1;
            } else {
                right += 1;
            }
        }
        current_max
    }
}
