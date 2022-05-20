use std::collections::HashMap;

#[derive(Default)]
pub struct EntryType(pub HashMap<String, f64>);

impl EntryType {
    pub fn new() -> EntryType {
        EntryType(HashMap::new())
    }

    pub fn merge(&mut self, other: &EntryType) {
        for (k, v) in other.0.iter() {
            *self.0.entry(k.clone()).or_insert(0.0) += *v;
        }
    }

    pub fn sum(&self) -> f64 {
        self.0.values().sum::<f64>()
    }

    pub fn add_value(&mut self, name: &str, value: f64) {
        if value > 0.0 {
            *self.0.entry(String::from(name)).or_insert(0.0) += value;
        }
    }
}