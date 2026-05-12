use std::collections::HashMap;

pub fn log_parser(values: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for string in values.iter() {
        if let Some((user_id, message)) = string.split_once(":") {
            map.entry(user_id.to_string())
                .or_insert(vec![])
                .push(message.to_string());
        }
    }
    map   
}
