use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;

use wasm_bindgen::prelude::*;


static mut EDGE_ID: i32 = 0;

pub type EdgeFunction = Box<dyn Fn(&mut Node) -> (String, f64)>;

pub struct Edge {
    pub from: Rc<RefCell<Node>>,
    pub to: Rc<RefCell<Node>>,
    pub value_func: EdgeFunction,
    pub id: i32,
}

impl Edge {
    pub fn new(from: Rc<RefCell<Node>>, to: Rc<RefCell<Node>>, value_func: EdgeFunction) -> Rc<RefCell<Edge>> {
        unsafe {
            let edge = Edge {
                from,
                to,
                value_func,
                id: EDGE_ID,
            };

            EDGE_ID += 1;

            Rc::new(RefCell::new(edge))
        }
    }
}

pub struct Node {
    pub values: HashMap<String, f64>,
    pub output_edges: HashMap<i32, Rc<RefCell<Edge>>>,
    pub input_edges: HashMap<i32, Rc<RefCell<Edge>>>,
    pub dirty: bool,
    pub cached_value: f64,
}

impl Node {
    pub fn new_empty() -> Rc<RefCell<Node>> {
        let node = Node {
            values: HashMap::new(),
            output_edges: HashMap::new(),
            input_edges: HashMap::new(),
            dirty: false,
            cached_value: 0.0,
        };

        Rc::new(RefCell::new(node))
    }

    pub fn new(initial_value: f64) -> Rc<RefCell<Node>> {
        let node: Rc<RefCell<Node>> = Node::new_empty();
        if initial_value != 0.0 {
            (*node).borrow_mut().set_value_without_set_dirty("初始值", initial_value);
            (*node).borrow_mut().cached_value = initial_value;
        }

        node
    }

    pub fn set_value(&mut self, key: &str, value: f64) {
        self.set_value_without_set_dirty(key, value);
        self.set_dirty();
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
        for output_edge in self.output_edges.iter() {
            let mut edge = output_edge.1;
            edge.borrow_mut().to.borrow_mut().set_dirty();
        }
    }

    fn set_value_without_set_dirty(&mut self, key: &str, value: f64) {
        *self.values.entry(key.to_string()).or_insert(0.0) = value;
    }

    fn calc_value(&self) -> f64 {
        match self.values.values().cloned().reduce(|a, b| { a + b }) {
            Some(v) => v,
            None => 0.0,
        }
    }

    pub fn value(&mut self) -> f64 {
        if !self.dirty {
            return self.cached_value;
        }

        self.dirty = false;
        let input_edges: Vec<Rc<RefCell<Edge>>> = self.input_edges.iter().map(|x| Rc::clone(x.1)).collect();
        for input_edge in input_edges {
            let input_node = &(*input_edge).borrow().from;
            let value_func = &(*input_edge).borrow().value_func;
            let (key_from_node, value_from_node) = (*value_func)(&mut input_node.borrow_mut());
            self.set_value_without_set_dirty(key_from_node.as_str(), value_from_node);
        }
        self.cached_value = self.calc_value();

        self.cached_value
    }

    pub fn add_edge(from: Rc<RefCell<Node>>, to: Rc<RefCell<Node>>, value_func: EdgeFunction) -> i32 {
        let edge = Edge::new(
            Rc::clone(&from),
            Rc::clone(&to),
            value_func
        );

        let edge_id = edge.borrow().id;

        let mut from_node_ref = (*from).borrow_mut();
        (*from_node_ref).output_edges.insert(edge_id, Rc::clone(&edge));

        let mut to_node_ref = (*to).borrow_mut();
        (*to_node_ref).input_edges.insert(edge_id, Rc::clone(&edge));
        to_node_ref.set_dirty();

        edge_id
    }
}

pub struct NodeHandle {
    node: Rc<RefCell<Node>>,
}

impl NodeHandle {
    pub fn new(initial_value: f64) -> NodeHandle {
        let node = Node::new(initial_value);
        NodeHandle {
            node
        }
    }

    pub fn set_value(&mut self, key: &str, value: f64) {
        (*self.node).borrow_mut().set_value(key, value);
    }

    pub fn get_value_by_key(&self, key: &str) -> f64 {
        let x = &(*self.node).borrow().values;
        match x.get(key) {
            Some(v) => *v,
            None => 0.0
        }
    }

    pub fn value(&self) -> f64 {
        (*self.node).borrow_mut().value()
    }

    pub fn add_edge(&mut self, to: &NodeHandle, value_func: EdgeFunction) -> i32 {
        Node::add_edge(
            Rc::clone(&self.node),
            Rc::clone(&to.node),
            value_func
        )
    }

    pub fn remove_edge(&mut self, id: i32) {
        let edge = Rc::clone(self.node.borrow().output_edges.get(&id).unwrap());
        let target_node = &edge.borrow().to;

        self.node.borrow_mut().output_edges.remove(&id);
        target_node.borrow_mut().input_edges.remove(&id);
    }

    pub fn get_composition(&self) -> HashMap<String, f64> {
        self.node.borrow().values.clone()
    }
}