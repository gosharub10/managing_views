use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("{},{}", title, result);
            }
            None => println!("not found a {}", title),
        }
    }
}
