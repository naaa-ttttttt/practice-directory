use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut start = 0;
    let mut freq_map = HashMap::new();
    let mut max_substring = 0;
    let mut max_freq = 0;
    let string: Vec<char> = s.chars().collect();

    // so we loop through every element, and then store them and there
    // values in the hashmap, and then we check, for every value, if 
    // the value has a hifher count, we use it to perform the operations
    for end in 0..string.len() {
        let end_value = string[end];
        let count = freq_map.entry(end_value).or_insert(0);
        *count += 1;
        max_freq = max_freq.max(*count);

        while (end - start) as i32 - max_freq < k {
            let left_char = string[start];
        }
    } 
    0
}
