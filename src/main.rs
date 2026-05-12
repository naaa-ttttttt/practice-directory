use rust_practice::practice;
fn main() {
    let chats: Vec<String> = vec!["1:nathaniel".to_string(), "2:noel".to_string(), "3:nelson".to_string(), "4:nicolas".to_string()];
    practice::log_parser(chats);
    println!("Hello, world!");
}
