#![feature(nll)]

use std::collections::HashMap;
use std::sync::{Mutex, Arc};

pub(crate) struct Connection {
    pub(crate) to_element: String,
    pub(crate) score: i8
}

pub(crate) trait Connector<S> {
    fn connect<T>(&mut self, con_handler: T) where T: Fn(&String, &mut S);
}

impl Connector<HashMap<String, Vec<Connection>>> for HashMap<String, Vec<Connection>> {
    fn connect<T>(&mut self, con_handler: T) where T: Fn(&String, &mut HashMap<String, Vec<Connection>>) {
        let keys= self.keys().cloned().collect::<Vec<_>>();
        keys.iter().for_each(|id| con_handler(id, self));
    }
}

impl Connector<HashMap<String, Vec<Connection>>> for Arc<Mutex<HashMap<String, Vec<Connection>>>> {
    fn connect<T>(&mut self, con_handler: T) where T: Fn(&String, &mut HashMap<String, Vec<Connection>>) {
        self.lock().unwrap().connect(con_handler);
    }
}