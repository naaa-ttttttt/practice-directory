fn three_sum(num: Vec<i32>) -> Vec<Vec<i32>> {
    num.sort();
    let mut result = Vec::new();
    let len = num.len();

    for i in 0..len{
        if i > 0 && num[i] == num[i - 1] {
            i += 1;
        }

        let left = i + 1; //this is the element after the first element.
        let right = len - 1; // this is the last element on the list.

        while left < right {
            let sum = num[i] + nums[left] + nums[right];

            if sum < 0 {
                left += 1;
            }
        }
    }
}
