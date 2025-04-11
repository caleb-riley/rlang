use std::collections::HashMap;

use crate::FnObj;

pub struct ObjRegistry {
    funcs: HashMap<String, FnObj>,
}

impl ObjRegistry {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    pub fn register_func(&mut self, name: String, func: FnObj) {
        self.funcs.insert(name, func);
    }

    pub fn get_func(&self, name: &String) -> Option<&FnObj> {
        self.funcs.get(name)
    }
}
