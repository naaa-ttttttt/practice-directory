fn sort_colors(num: &mut Vec<i32>) {
    let length = num.len();
    let mut left = 0;
    let mut right = length - 1;
    let mut i = 0;

    while i <= right {
        if num[i] == 0 {
            num.swap(i, left);
            left += 1;
        } 

    }
    
}
