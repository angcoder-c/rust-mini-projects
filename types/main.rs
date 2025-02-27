use std::collections::HashMap;

fn main() {
    let mut concurrence: HashMap<&str, Vec<u32>> = HashMap::new();
    let randomString : String = String::from("abbcccdddd");

    for item in randomString.chars() {
        println!("{}", item);
    }
}