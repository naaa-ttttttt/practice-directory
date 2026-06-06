fn triangle_num(mut num: Vec<i32>) -> i32 {
    num.sort();
    let mut count = 0;
    let length = num.len();

    for i in (0..length).rev() {
        let mut left = 0;
        let mut right = i - 1;

        while left < right {
            if num[left] + num[right] > i.try_into().unwrap() {
                count += right - left;
                right -= 1;
            } else {
                left += 1;
            }
        }
    }

    count.try_into().unwrap()
}
