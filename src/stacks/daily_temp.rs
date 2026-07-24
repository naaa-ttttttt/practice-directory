pub fn daily_temp(temp: Vec<i32>) -> Vec<i32> {
    let len = temp.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut result = vec![0; len];

    for i in 0..len {
        while !stack.is_empty() && temp[i] > temp[*stack.last().unwrap()] {
            let idx = stack.pop().unwrap();
            result[idx] = (i - idx) as i32;
        } 
        stack.push(i);
    }
        


    result
}
