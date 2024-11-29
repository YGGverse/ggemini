use super::Connection;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Request sessions holder for `Client` object
/// * useful to keep connections open and / or validate TLS certificate updates in runtime
pub struct Session {
    index: RefCell<HashMap<String, Rc<Connection>>>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            index: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, request: &str) -> Option<Rc<Connection>> {
        self.index.borrow().get(request).cloned()
    }

    pub fn update(&self, request: String, connection: Rc<Connection>) -> Option<Rc<Connection>> {
        self.index.borrow_mut().insert(request, connection)
    }
}
