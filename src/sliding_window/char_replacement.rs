use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut start = 0;
    let mut freq_map = HashMap::new();
    let mut max_substring = 0;
    let max_freq = 0;
    let string: Vec<char> = s.chars().collect();

    // so we loop through every element, and then store them and there
    // values in the hashmap, and then we check, for every value, if 
    // the value has a hifher count, we use it to perform the operations
    for end in 0..string.len() {
        let end_value = string[end];
        *freq_map.entry(end_value).or_insert(0) += 1;

        //if freq_map[&end_value]   
    } 
    0
}
