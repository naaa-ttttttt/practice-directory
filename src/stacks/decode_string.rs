pub fn decode_strings(s: String) -> String {
        // Stack-based approach: handle nested brackets for string decoding
        let mut stack: Vec<(String, i32)> = Vec::new();
        let mut curr_string = String::new();    // Current string being built
        let mut current_number = 0;             // Current multiplier number
        
        for ch in s.chars() {
            match ch {
                '[' => {
                    // Start of new encoded section: save current state
                    stack.push((curr_string.clone(), current_number));
                    curr_string.clear();        // Reset for new section
                    current_number = 0;
                }
                ']' => {
                    // End of encoded section: decode and merge
                    let (prev_string, num) = stack.pop().unwrap();
                    
                    // Repeat current string num times
                    let repeated = curr_string.repeat(num as usize);
                    curr_string = prev_string + &repeated;
                }
                '0'..='9' => {
                    // Digit: build the multiplier number
                    current_number = current_number * 10 + ch.to_digit(10).unwrap() as i32;
                }
                _ => {
                    // Letter: add to current string
                    curr_string.push(ch);
                }
            }
        }
        
        curr_string
}
