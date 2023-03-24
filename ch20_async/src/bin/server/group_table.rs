use group::Group;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct GroupTable(Mutext<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

   pub fn get(&self, name: &String) -> Option<Arc<Group>> {
       self.0.lock()
        .unwrap()
        .get(name)
        .cloned();
   }

   pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
       self.0.lock()
        .unwrap()
        .entry(name.clone())
        .or_insert_with(|| Arc::new(Group::new(name)))
        .clone()
   }
}
