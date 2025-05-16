use std::collections::HashMap;

pub struct Context {
    pub memory: String,
    pub todo_list: HashMap<String, Vec<String>>,
}