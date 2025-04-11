use std::collections::HashMap;

use crate::value::Value;

pub struct Scope {
    pub variables: HashMap<String, Value>,
    pub child: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            child: None,
        }
    }

    pub fn push_scope(&mut self) {
        if let Some(child) = self.child.as_mut() {
            child.push_scope();
        } else {
            self.child = Some(Box::new(Scope::new()));
        }
    }

    pub fn pop_scope(&mut self) {
        if let Some(child) = self.child.as_mut() {
            if child.child.is_some() {
                child.pop_scope();
            } else {
                self.child = None;
            }
        }
    }

    fn resolve(&self, name: &String) -> Option<&Self> {
        if let Some(child) = &self.child {
            if let Some(scope) = child.resolve(name) {
                return Some(scope);
            }
        }

        if self.variables.contains_key(name) {
            return Some(self);
        }

        None
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        let scope = self.resolve(name)?;
        scope.variables.get(name)
    }

    pub fn set(&mut self, name: String, obj: Value) {
        use std::collections::hash_map;

        if let hash_map::Entry::Occupied(mut e) =
            self.variables.entry(name.clone())
        {
            e.insert(obj);
        } else if let Some(child) = self.child.as_mut() {
            child.set(name, obj);
        } else {
            panic!("Cannot update a variable that hasn't been declared");
        }
    }

    pub fn declare(&mut self, name: String, obj: Value) {
        if let Some(child) = self.child.as_mut() {
            child.declare(name, obj);
        } else {
            self.variables.insert(name, obj);
        }
    }
}
