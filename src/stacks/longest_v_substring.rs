pub fn longest_v_substring(s: String) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    let count = 0;

    for ch in s.chars() {
        if ch == '(' {
            stack.push(ch);
        }
    }

    count
}
