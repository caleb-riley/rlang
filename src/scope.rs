use std::collections::HashMap;

use crate::value::Value;

pub struct ScopeHolder {
    scope: Option<Scope>,
}

impl ScopeHolder {
    pub fn new() -> Self {
        Self { scope: None }
    }

    pub fn push_scope(&mut self) {
        let current_scope = self.scope.take();

        self.scope = Some(Scope::new(current_scope));
    }

    pub fn pop_scope(&mut self) -> Result<(), ()> {
        let current_scope = self.scope.take().ok_or(())?;

        self.scope = current_scope.parent.map(|s| *s);

        Ok(())
    }

    pub fn inner(&self) -> Option<&Scope> {
        self.scope.as_ref()
    }

    pub fn inner_mut(&mut self) -> Option<&mut Scope> {
        self.scope.as_mut()
    }
}

pub struct Scope {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    fn new(parent: Option<Self>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: parent.map(Box::new),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let var @ Some(_) = self.variables.get(name) {
            return var;
        }

        match self.parent {
            Some(ref parent) => parent.get(name),
            None => None,
        }
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        if let var @ Some(_) = self.variables.get_mut(name) {
            return var;
        }

        match self.parent {
            Some(ref mut parent) => parent.get_mut(name),
            None => None,
        }
    }

    pub fn set(&mut self, name: &str, obj: Value) -> Result<(), ()> {
        self.get_mut(name).map(|v| *v = obj).ok_or(())
    }

    pub fn declare(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}
