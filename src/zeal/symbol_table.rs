use std::collections::HashMap;

pub struct SymbolTable {
    label_map: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> Self{
        SymbolTable {
            label_map: HashMap::new()
        }
    }

    pub fn add_or_update_label(&mut self, label_name: &str, address: u32) {
        self.label_map.insert(label_name.to_owned(), address);
    }

    pub fn address_for(&self, label_name: &str) -> u32 {
        match self.label_map.get(label_name) {
            Some(&address) => address,
            None => 0,
        }
    }

    pub fn has_label(&self, label_name: &str) -> bool {
        self.label_map.contains_key(label_name)
    }
}
