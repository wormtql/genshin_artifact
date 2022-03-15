use std::cell::RefCell;
use super::attribute::Attribute;
use super::typing::EdgeFunctionFwd;

use super::attribute_name::AttributeName;

const MAX_EDGE_COUNT: usize = 20;

#[derive(Copy, Clone, Debug)]
pub struct SimpleEntry {
    pub value_from_edge: f64,
    pub value_self: f64,
    pub dirty: bool,
    pub cached_value: f64,
}

pub struct SimpleEdge {
    pub func: EdgeFunctionFwd,
    pub from: usize,
    pub to: usize,
}

#[deprecated(note = "use priority based SimpleAttributeGraph2 instead")]
pub struct SimpleAttributeGraph {
    pub attributes: RefCell<[SimpleEntry; 150]>,
    // pub values: [f64; 150],
    // pub output_edges: HashMap<usize, Vec<(Rc<EdgeFunction>, usize)>>,
    // pub output_edges: HashMap<usize, Vec<Rc<SimpleEdge>>>,
    // pub input_edges: HashMap<usize, Vec<Rc<SimpleEdge>>>
    pub output_edges: [[usize; MAX_EDGE_COUNT]; 150],
    pub output_edge_size: [i32; 150],
    pub input_edges: [[usize; MAX_EDGE_COUNT]; 150],
    pub input_edge_size: [i32; 150],

    edges: Vec<SimpleEdge>,
}

impl Default for SimpleAttributeGraph {
    fn default() -> Self {
        let mut temp = SimpleAttributeGraph {
            attributes: RefCell::new([SimpleEntry {
                value_from_edge: 0.0,
                value_self: 0.0,
                dirty: false,
                cached_value: 0.0,
            }; 150]),
            // values: [0.0; 150],
            // output_edges: HashMap::new(),
            // input_edges: HashMap::new(),
            output_edges: [[0; MAX_EDGE_COUNT]; 150],
            output_edge_size: [0; 150],
            input_edges: [[0; MAX_EDGE_COUNT]; 150],
            input_edge_size: [0; 150],
            edges: Vec::new(),
        };

        let ptr = temp.attributes.as_ptr();
        unsafe {
            (*ptr)[AttributeName::Recharge as usize].value_self = 1.0;
            (*ptr)[AttributeName::Recharge as usize].cached_value = 1.0;
            (*ptr)[AttributeName::CriticalBase as usize].value_self = 0.05;
            (*ptr)[AttributeName::CriticalBase as usize].cached_value = 0.05;
            (*ptr)[AttributeName::CriticalDamageBase as usize].value_self = 0.5;
            (*ptr)[AttributeName::CriticalDamageBase as usize].cached_value = 0.5;
        }

        temp
    }
}

impl Attribute for SimpleAttributeGraph {
    type EdgeHandle = ();

    fn get_value(&self, key: AttributeName) -> f64 {
        self.my_get_value(key as usize)
    }

    fn set_value_to(&mut self, name: AttributeName, _key: &str, value: f64) {
        let ptr = self.attributes.as_ptr();
        unsafe {
            (*ptr)[name as usize].value_self = value;
        }
        // self.attributes[name as usize].borrow_mut().value_self = value;
        self.set_dirty(name as usize);
    }

    fn set_value_by(&mut self, name: AttributeName, _key: &str, value: f64) {
        let ptr = self.attributes.as_ptr();
        unsafe {
            (*ptr)[name as usize].value_self += value;
        }
        // self.attributes[name as usize].value_self += value;
        // self.attributes[name as usize].borrow_mut().value_self += value;
        self.set_dirty(name as usize);
    }

    fn add_edge(&mut self, from: AttributeName, to: AttributeName, edge: EdgeFunctionFwd, _priority: usize) -> Self::EdgeHandle {
        // self.output_edges.entry(from as usize).or_insert(Vec::new()).push((edge, to as usize));
        //
        // let x = self.output_edges.get(&(from as usize)).unwrap().len();
        //
        // (from as usize) * 1000 + x

        let temp = SimpleEdge {
            func: edge,
            from: from as usize,
            to: to as usize,
        };
        // self.output_edges.entry(from as usize).or_insert(Vec::new()).push(Rc::clone(&temp));
        // self.input_edges.entry(to as usize).or_insert(Vec::new()).push(Rc::clone(&temp));
        //
        // let x = self.output_edges.get(&(from as usize)).unwrap().len() - 1;
        // (from as usize) * 1000 + x

        let from = from as usize;
        let to = to as usize;
        self.edges.push(temp);
        let edge_handle = self.edges.len() - 1;

        self.output_edges[from][self.output_edge_size[from] as usize] = edge_handle;
        self.output_edge_size[from] = (self.output_edge_size[from] + 1) % MAX_EDGE_COUNT as i32;
        self.input_edges[to][self.input_edge_size[to] as usize] = edge_handle;
        self.input_edge_size[to] = (self.input_edge_size[to] + 1) % MAX_EDGE_COUNT as i32;
    }

    fn remove_edge(&mut self, _handle: Self::EdgeHandle) {
        // let from = handle / 1000;
        // let index = handle % 1000;
        //
        // self.output_edges.get_mut(&from).unwrap().remove(index);
    }
}

impl SimpleAttributeGraph {
    pub fn set_dirty(&self, index: usize) {
        let ptr = self.attributes.as_ptr();

        unsafe {
            (*ptr)[index].dirty = true;
        }
        // self.attributes[index].borrow_mut().dirty = true;
        // if let Some(x) = self.output_edges.get(&index) {
        //     for edge in x.iter() {
        //         self.set_dirty(edge.to);
        //     }
        // }
        for i in 0..self.output_edge_size[index] {
            let edge = &self.edges[self.output_edges[index][i as usize]];
            self.set_dirty(edge.to);
        }
    }

    pub fn my_get_value(&self, index: usize) -> f64 {
        let ptr = self.attributes.as_ptr();

        let temp = unsafe {
            &mut (*ptr)[index]
        };
        if temp.dirty {
            // if let Some(x) = self.input_edges.get(&index) {
            //     temp.value_from_edge = 0.0;
            //     for i in x.iter() {
            //         let from = i.from;
            //         let from_value = self.my_get_value(from);
            //         let to_value = (*i.func)(from_value).1;
            //         temp.value_from_edge += to_value;
            //     }
            //     temp.cached_value = temp.value_self + temp.value_from_edge;
            // } else {
            //     temp.cached_value = temp.value_self;
            // }

            temp.value_from_edge = 0.0;
            for i in 0..self.input_edge_size[index] {
                let edge = &self.edges[self.input_edges[index][i as usize]];
                let from = edge.from;
                let from_value = self.my_get_value(from);
                let to_value = (edge.func)(from_value).1;
                temp.value_from_edge += to_value;
            }
            temp.cached_value = temp.value_self + temp.value_from_edge;
        }

        temp.cached_value
    }

    // pub fn build(&mut self) {
    //     for i in 0_usize..150 {
    //         self.attributes[i].value_from_edge = 0.0;
    //     }
    //
    //     for (&from, edge_function) in self.output_edges.iter() {
    //         for (ef, to) in edge_function.iter() {
    //             let from_value = self.attributes[from].value_self;
    //             let to_value: f64 = ef(from_value).1;
    //
    //             self.attributes[*to].value_from_edge += to_value;
    //         }
    //     }
    //
    //     for i in 0_usize..150 {
    //         self.values[i] = self.attributes[i].value_self + self.attributes[i].value_from_edge;
    //     }
    // }
}