pub fn longest_v_substring(s: String) -> i32 {
    let mut stack = vec![-1];
    let mut count = 0;

    for (i, ch) in s.chars().enumerate() {
        if ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            stack.pop().unwrap();
            count += 2;
        }
    }

    count
}
