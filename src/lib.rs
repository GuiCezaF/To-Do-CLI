// src/lib.rs

use std::collections::HashMap;

pub struct TodoList {
    // true = to do, false = done
    pub items: HashMap<String, bool>,
}

impl TodoList {
    pub fn new() -> TodoList {
        let items: HashMap<String, bool> = HashMap::new();
        TodoList { items }
    }

    // Método de acesso para items
    pub fn items(&self) -> &HashMap<String, bool> {
        &self.items
    }
}
