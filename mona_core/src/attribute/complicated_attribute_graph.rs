use std::cell::RefCell;
use std::collections::HashMap;
use crate::attribute::typing::EdgeFunctionBwd;

use rand::Rng;
use crate::common::{Element, EntryType, SkillType};

use super::attribute_name::AttributeName;
use super::typing::EdgeFunctionFwd;
use super::attribute::Attribute;

pub struct MyEdge {
    pub from1: usize,
    pub from2: usize,
    pub to: usize,
    pub key: String,
    pub fwd: EdgeFunctionFwd,
    pub bwd: EdgeFunctionBwd,
    pub id: usize,
}

pub struct MyNode {
    pub value_self: HashMap<String, f64>,
    pub value_from_edge: HashMap<String, f64>,
    pub dirty: bool,
    pub cache_value: f64,
}

impl MyNode {
    pub fn sum(&self) -> f64 {
        self.value_self.values().sum::<f64>() + self.value_from_edge.values().sum::<f64>()
    }

    pub fn set_edge_value_by(&mut self, key: &str, value: f64) {
        *self.value_from_edge.entry(String::from(key)).or_insert(0.0) += value;
    }

    pub fn set_self_value_by(&mut self, key: &str, value: f64) {
        *self.value_self.entry(String::from(key)).or_insert(0.0) += value;
    }

    pub fn composition(&self) -> HashMap<String, f64> {
        let mut temp: HashMap<String, f64> = HashMap::new();
        for (k, v) in self.value_self.iter().chain(self.value_from_edge.iter()) {
            *temp.entry(k.clone()).or_insert(0.0) += *v;
        }

        temp
    }
}

const MAX_ATTRIBUTE_ENTRY: usize = 200;

pub struct ComplicatedAttributeGraph {
    pub attributes: RefCell<[MyNode; MAX_ATTRIBUTE_ENTRY]>,
    pub edges: Vec<MyEdge>,
}

impl Default for ComplicatedAttributeGraph {
    fn default() -> Self {
        let ret = ComplicatedAttributeGraph {
            attributes: RefCell::new([(); MAX_ATTRIBUTE_ENTRY].map(|_| MyNode {
                value_self: HashMap::new(),
                value_from_edge: HashMap::new(),
                dirty: true,
                cache_value: 0.0
            })),
            edges: Vec::new(),
        };

        let data = ret.attributes.as_ptr();
        unsafe {
            (*data)[AttributeName::CriticalBase as usize].set_self_value_by("初始值", 0.05);
            (*data)[AttributeName::CriticalDamageBase as usize].set_self_value_by("初始值", 0.5);
            (*data)[AttributeName::Recharge as usize].set_self_value_by("初始值", 1.0);
        }

        ret
    }
}

impl Attribute for ComplicatedAttributeGraph {
    type EdgeHandle = usize;

    fn get_value(&self, key: AttributeName) -> f64 {
        self.my_get_value(key as usize)
    }

    fn set_value_to(&mut self, name: AttributeName, key: &str, value: f64) {
        *self.attributes
            .borrow_mut()[name as usize]
            .value_self
            .entry(String::from(key))
            .or_insert(0.0) = value;
    }

    fn set_value_by(&mut self, name: AttributeName, key: &str, value: f64) {
        *self.attributes
            .borrow_mut()[name as usize]
            .value_self
            .entry(String::from(key))
            .or_insert(0.0) += value;
    }

    fn add_edge(
        &mut self,
        from1: usize,
        from2: usize,
        to: usize,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str
    ) -> Self::EdgeHandle {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();
        let edge = MyEdge {
            from1,
            from2,
            to,
            key: String::from(key),
            fwd,
            bwd,
            id,
        };

        self.edges.push(edge);
        id
    }

    fn remove_edge(&mut self, handle: Self::EdgeHandle) {
        let mut index = 0;
        for (i, edge) in self.edges.iter().enumerate() {
            if edge.id == handle {
                index = i;
                break;
            }
        }

        self.edges.remove(index);
    }
}

impl ComplicatedAttributeGraph {
    fn my_get_value(&self, index: usize) -> f64 {
        let data = self.attributes.as_ptr();
        let node = unsafe {
            &mut (*data)[index]
        };

        if node.dirty {
            node.value_from_edge.clear();
            for edge in self.edges.iter() {
                if edge.to == index {
                    let from_value1 = self.get_from_value(edge.from1);
                    let from_value2 = self.get_from_value(edge.from2);
                    let to_value: f64 = (edge.fwd)(from_value1, from_value2);
                    node.set_edge_value_by(&edge.key, to_value);
                }
            }

            node.cache_value = node.sum();
            node.dirty = false;
        }

        node.cache_value
    }

    fn get_from_value(&self, index: usize) -> f64 {
        if index == usize::MAX {
            0.0
        } else {
            self.my_get_value(index)
        }
    }

    pub fn get_attribute_composition(&self, name: AttributeName) -> EntryType {
        // to refresh dirty
        self.get_value(name);

        EntryType(self.attributes.borrow()[name as usize].composition())
    }

    pub fn get_composition_merge(&self, names: &[AttributeName]) -> EntryType {
        let mut temp = EntryType::new();
        for name in names.iter() {
            let comp = self.get_attribute_composition(*name);
            temp.merge(&comp);
        }

        temp
    }

    pub fn get_critical_composition(&self, element: Element, skill: SkillType) -> EntryType {
        let skill_type_critical_name = AttributeName::critical_rate_name_by_skill_type(skill);
        let mut names = vec![
            AttributeName::CriticalBase,
            AttributeName::CriticalAttacking,
            AttributeName::critical_rate_name_by_element(element),
        ];
        if let Some(name) = skill_type_critical_name {
            names.push(name);
        }

        self.get_composition_merge(&names)
    }
}