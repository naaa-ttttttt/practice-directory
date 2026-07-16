use std::collections::HashMap;

pub fn valid_parenthesis(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    let mapping = HashMap::from([
        (')', '('),
        ('}', '{'),
        (']', '['),
    ]);

    for ch in s.chars() {
        if let Some(&open_char) = mapping.get(&ch) {
            if stack.is_empty() || stack[stack.len() - 1] != open_char {
                return false;
            }
            stack.pop();
        } else {
            stack.push(ch);
        } 
    }
    stack.is_empty()
}
