pub struct Solution;

impl Solution {
    pub fn sum_two(num: Vec<i32>, target: i32) -> bool {
        let mut left = num[0];
        let mut right = num.len() -1;
        let mut current_sum = 0;

        for sum in num.iter() {
            if (left + right) as i32 == target {
                true;
            } else {
                false;
            }

            current_sum = sum;

            if current_sum == sum {
                true;
            } else {
                false;
            }
        }

    }
}
