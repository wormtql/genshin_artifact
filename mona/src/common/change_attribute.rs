use crate::attribute::AttributeGraph;

pub trait ChangeAttribute {
    fn change_attribute(&self, attribute: &mut AttributeGraph);
}

pub struct ChangeAttributeList {
    pub list: Vec<Box<dyn ChangeAttribute>>
}

impl ChangeAttributeList {
    pub fn new() -> ChangeAttributeList {
        ChangeAttributeList {
            list: Vec::new()
        }
    }

    pub fn add(&mut self, item: Box<dyn ChangeAttribute>) {
        self.list.push(item);
    }
}

impl ChangeAttribute for ChangeAttributeList {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        for item in self.list.iter() {
            item.change_attribute(attribute);
        }
    }
}