use std::collections::HashMap;

use crate::value::Value;

/// Holds a scope to allow push and pop operations.
#[derive(Default)]
pub struct ScopeManager {
    /// The optional current scope, if it exists.
    scope: Option<Scope>,
}

impl ScopeManager {
    /// Adds a new level to the scope hierarchy.
    pub fn push_scope(&mut self) {
        let current_scope = self.scope.take();

        self.scope = Some(Scope::new(current_scope));
    }

    /// Removes the last level from the scope hierarchy.
    pub fn pop_scope(&mut self) -> Result<(), ()> {
        let current_scope = self.scope.take().ok_or(())?;

        self.scope = current_scope.parent.map(|s| *s);

        Ok(())
    }

    /// Returns a shared reference to the inner scope.
    pub fn inner(&self) -> Option<&Scope> {
        self.scope.as_ref()
    }

    // Returns an exclusive reference to the inner scope.
    pub fn inner_mut(&mut self) -> Option<&mut Scope> {
        self.scope.as_mut()
    }
}

/// Holds the variables present at the current level of execution.
pub struct Scope {
    /// A hash map storing the variables as key value pairs.
    variables: HashMap<String, Value>,

    /// The optional parent of this scope, inherits values from it.
    parent: Option<Box<Scope>>,
}

impl Scope {
    /// Creates a new scope, given the optional parent scope.
    fn new(parent: Option<Self>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: parent.map(Box::new),
        }
    }

    /// Returns a shared reference to the value with the given name.
    pub fn get(&self, var_name: &str) -> Option<&Value> {
        if let var @ Some(_) = self.variables.get(var_name) {
            return var;
        }

        match self.parent {
            Some(ref parent) => parent.get(var_name),
            None => None,
        }
    }

    /// Returns an exclusive reference to the value with the given name.
    fn get_mut(&mut self, var_name: &str) -> Option<&mut Value> {
        if let value @ Some(_) = self.variables.get_mut(var_name) {
            return value;
        }

        match self.parent {
            Some(ref mut parent) => parent.get_mut(var_name),
            None => None,
        }
    }

    /// Sets the value of the variable with the given name to the given value.
    pub fn set(&mut self, var_name: &str, new_value: Value) -> Result<(), ()> {
        self.get_mut(var_name)
            .map(|value| *value = new_value)
            .ok_or(())
    }

    /// Declares a new variable with the given name in the current scope, giving
    /// it the initial provided value.
    pub fn declare(&mut self, var_name: String, initial_value: Value) {
        self.variables.insert(var_name, initial_value);
    }
}
