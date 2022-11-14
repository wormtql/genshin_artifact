use smallvec::SmallVec;
use std::cell::RefCell;
use crate::attribute::typing::EdgeFunctionBwd;

use super::attribute::Attribute;
use super::typing::EdgeFunctionFwd;

use super::attribute_name::AttributeName;

const MAX_EDGE_COUNT: usize = 30;
const MAX_NODE_COUNT: usize = 200;

#[derive(Copy, Clone, Debug)]
struct SimpleEntry {
    pub value_from_edge: f64,
    pub value_self: f64,
    pub cached_value: f64,
    pub dirty: bool,
}

struct SimpleEdge {
    pub func: EdgeFunctionFwd,
    pub from1: usize,
    pub from2: usize,
    pub to: usize,
    pub key: String,
}

pub struct SimpleAttributeGraph2 {
    attributes: RefCell<[SimpleEntry; MAX_NODE_COUNT]>,
    // edges: Vec<Rc<SimpleEdge>>,
    // edges: Vec<SimpleEdge>,
    edges: SmallVec<[SimpleEdge; MAX_EDGE_COUNT]>,
    pub set_dirty_on_set_value: bool,

    // atk_percentage: f64,
    // def_percentage: f64,
    // hp_percentage: f64,
}

impl Default for SimpleAttributeGraph2 {
    fn default() -> Self {
        let temp = SimpleAttributeGraph2 {
            attributes: RefCell::new([SimpleEntry {
                value_from_edge: 0.0,
                value_self: 0.0,
                cached_value: 0.0,
                dirty: true
            }; MAX_NODE_COUNT]),
            // edges: Vec::new(),
            edges: SmallVec::new(),
            set_dirty_on_set_value: false,
            // atk_percentage: 0.0,
            // def_percentage: 0.0,
            // hp_percentage: 0.0,
        };

        let data = temp.attributes.as_ptr();

        unsafe {
            (*data)[AttributeName::CriticalBase as usize].value_self = 0.05;
            (*data)[AttributeName::CriticalDamageBase as usize].value_self = 0.5;
            (*data)[AttributeName::Recharge as usize].value_self = 1.0;
        }

        temp
    }
}

impl Attribute for SimpleAttributeGraph2 {
    type EdgeHandle = ();

    fn get_value(&self, key: AttributeName) -> f64 {
        self.my_get_value(key as usize)
    }

    fn set_value_to(&mut self, name: AttributeName, _key: &str, value: f64) {
        let data = self.attributes.as_ptr();
        unsafe {
            (*data)[name as usize].value_self = value;
        }
        if self.set_dirty_on_set_value {
            self.mark_dirty(name as usize);
        }
    }

    fn set_value_by(&mut self, name: AttributeName, _key: &str, value: f64) {
        let data = self.attributes.as_ptr();
        unsafe {
            (*data)[name as usize].value_self += value;
        }
        if self.set_dirty_on_set_value {
            self.mark_dirty(name as usize);
        }
    }

    fn add_edge(
        &mut self,
        from1: usize,
        from2: usize,
        to: usize,
        fwd: EdgeFunctionFwd,
        _bwd: EdgeFunctionBwd,
        key: &str
    ) -> Self::EdgeHandle {
        let edge = SimpleEdge {
            func: fwd,
            from1,
            from2,
            to,
            key: String::from(key)
        };
        // self.edges.push(Rc::new(edge));
        self.edges.push(edge);
    }

    fn remove_edge(&mut self, _handle: Self::EdgeHandle) {
    }
}

impl SimpleAttributeGraph2 {
    fn my_get_value(&self, index: usize) -> f64 {
        // println!("get value: {}", index);
        let data = self.attributes.as_ptr();
        let node = unsafe {
            &mut (*data)[index]
        };
        if node.dirty {
            node.value_from_edge = 0.0;
            for edge in self.edges.iter() {
                if edge.to == index {
                    let from_value1 = self.get_from_value(edge.from1);
                    let from_value2 = self.get_from_value(edge.from2);
                    // let from_value1 = 0.0;
                    // let from_value2 = 0.0;
                    let to_value: f64 = (edge.func)(from_value1, from_value2);
                    node.value_from_edge += to_value;
                }
            }
        }
        node.cached_value = node.value_self + node.value_from_edge;
        node.dirty = false;

        node.cached_value
        // 0.0
    }

    fn mark_dirty(&self, index: usize) {
        let data = self.attributes.as_ptr();
        let node = unsafe {
            &mut (*data)[index]
        };
        node.dirty = true;
        for edge in self.edges.iter() {
            if edge.from1 == index || edge.from2 == index {
                let to_node = unsafe { &(*data)[edge.to] };
                if !to_node.dirty {
                    self.mark_dirty(edge.to);
                }
            }
        }
    }

    fn get_from_value(&self, index: usize) -> f64 {
        if index == usize::MAX {
            0.0
        } else {
            self.my_get_value(index)
        }
    }

    // pub fn build(&mut self) {
    //     // let atk_base = self.attributes[AttributeName::ATKBase as usize].value_self;
    //     // let def_base = self.attributes[AttributeName::DEFBase as usize].value_self;
    //     // let hp_base = self.attributes[AttributeName::HPBase as usize].value_self;
    //     //
    //     // self.attributes[AttributeName::ATKPercentage as usize].value_from_edge += atk_base * self.atk_percentage;
    //     // self.attributes[AttributeName::DEFPercentage as usize].value_from_edge += def_base * self.def_percentage;
    //     // self.attributes[AttributeName::HPPercentage as usize].value_from_edge += hp_base * self.hp_percentage;
    //
    //     self.edges.sort_by(|x, y| x.priority.cmp(&y.priority));
    //     for edge in self.edges.iter() {
    //         let from_value = self.attributes[edge.from].value_self + self.attributes[edge.from].value_from_edge;
    //         let to_value = (edge.func)(from_value).1;
    //         self.attributes[edge.to].value_from_edge += to_value;
    //     }
    // }

    // topological sort implementation
    // pub fn build(&mut self) {
    //     let mut in_degree: [usize; 150] = [0; 150];
    //     let mut related_node: HashSet<usize> = HashSet::new();
    //     for edge in self.edges.iter() {
    //         in_degree[edge.to] += 1;
    //         if edge.from2 != usize::MAX {
    //             in_degree[edge.to] += 1;
    //         }
    //         related_node.insert(edge.from1);
    //         if edge.from2 != usize::MAX {
    //             related_node.insert(edge.from2);
    //         }
    //         related_node.insert(edge.to);
    //     }
    //
    //     let mut queue: Vec<usize> = vec![];
    //     for &i in related_node.iter() {
    //         if in_degree[i] == 0 {
    //             queue.push(i);
    //         }
    //     }
    //
    //     while !queue.is_empty() {
    //         let p = queue.pop().unwrap();
    //
    //         let from_value = self.attributes[p].value_self + self.attributes[p].value_from_edge;
    //
    //         for edge in self.edges.iter() {
    //             if edge.from == p {
    //                 in_degree[edge.to] -= 1;
    //                 if in_degree[edge.to] == 0 {
    //                     queue.push(edge.to);
    //                 }
    //
    //                 let to_value = (edge.func)(from_value).1;
    //                 self.attributes[edge.to].value_from_edge += to_value;
    //             }
    //         }
    //     }
    // }
}