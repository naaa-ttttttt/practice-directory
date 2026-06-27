use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut start = 0;
    let mut freq_map = HashMap::new();
    let mut max_substring = 0;
    let string: Vec<char> = s.chars().collect();

    for end in 0..string.len() {
        let end_value = string[end];
        *freq_map.entry(end_value).or_insert(0) += 1;
    } 
    0
}
