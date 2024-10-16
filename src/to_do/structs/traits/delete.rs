use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file(String::from("./state.json"), state);
        println!("deleted");
    }
}
