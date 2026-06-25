use std::collections::HashMap;

pub fn longest_substring(words: String) -> i32 {
    let mut left = 0;
    let mut freq_map = HashMap::new();
    let mut longest_substring = 0;
    
    for chars in words.chars() {
        *freq_map.entry(chars).or_insert(0) += 1;
        longest_substring = freq_map.len();
        
        // here we are to check the count of the characters, 
        // and if any characters count is more than one,
        // we remove the element from the left, and then 
        // check then move the left pointer forward, 
        // rerun the code, i guess.

        if  
        
    }
    0
}
