use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;

pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file(String::from("./state.json"), state);
        println!("done");
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file(String::from("./state.json"), state);
        println!("pending");
    }
}
