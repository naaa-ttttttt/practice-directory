use std::collections::HashMap;

pub fn longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut freq_map = HashMap::new();
    let mut longest_substring = 0;
    let words: Vec<char> = s.chars().collect();
    
    // so we are to loop through the index of the values, 
    // then get the values at the right pointer, so we can store
    // it in the hashmap, and so while the count of the values is greater
    // than one, we should we remove the element at the left pointer,
    // and if the.
    for chars in 0..words.len() {
        let cha = words[chars];
        *freq_map.entry(&words[chars]).or_insert(0) += 1;
        
        // here we are to check the count of the characters, 
        // and if any characters count is more than one,
        // we remove the element from the left, and then 
        // check then move the left pointer forward, 
        // rerun the code, i guess.

        while 
        if *freq_map.get(&chars).unwrap() > 1 {
            words
            freq_map.remove(&words[left]);
        }  
        
    }
    0
}
