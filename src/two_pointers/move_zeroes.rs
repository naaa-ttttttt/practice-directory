fn move_zeroes(arr: &mut Vec<i32>) {
    let mut second_ptr = 0;
    let length = arr.len();

    for i in 0..length {
        if arr[i] == 0 {
            continue;
        } else {
            arr.swap(i, second_ptr);
            second_ptr += 1;
        }
    } 

}
