mod boxed_slice;
mod ustr_extern;
pub use boxed_slice::BoxedSlice;

use std::ffi::c_char;

mod hash_wrapper;

use std::collections::HashMap;

pub use crate::{Model, MetaData, Node, Link, Constant, NodeId};

type OptionPtr<T> = *const T;
fn none_ptr<T>() -> OptionPtr<T> { std::ptr::null() }
fn some_ptr<T>(t: &T) -> *const T { t as *const T }
fn option_to_ptr(option: Option<&Node>) -> OptionPtr<Node> {
    match option {
        Some(node) => some_ptr(node),
        None => none_ptr(),
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Constant;
        pub fn name(self: &Constant) -> &str ;
        pub fn value(self: &Constant) -> f64 ;
        type Model;
        pub fn add_node(self: &mut Model, node: Box<Node>);
        pub fn add_constant(self: &mut Model, name: &str, value: f64);
        pub fn meta_data(self: &Model) -> &MetaData ;
        pub fn constants(self: &Model) -> &[Constant] ;
        pub fn get_node(self: &Model, id: u32) -> *const Node ;

        type MetaData;
        pub fn start_time(self: &MetaData) -> f64 ;
        pub fn end_time(self: &MetaData) -> f64 ;
        pub fn delta_time(self: &MetaData) -> f64 ;
        type Node;
        pub fn add_link(self: &mut Node, sign: u32, node_id: u32);
        pub fn add_input(self: &mut Node, input: u32);
        pub fn id(self: &Node) -> &u32 ;
        pub fn name(self: &Node) -> &str ;
        pub fn related_constant_name(self: &Node) -> &str ;
        pub fn operation(self: &Node) -> *const u32 ;
        type Link;
        pub fn sign(self: &Link) -> u32 ;
        pub fn node_id(self: &Link) -> u32 ;


        pub fn new_model(start_time: f64, end_time: f64, delta_time: f64) -> Box<Model>;
        pub fn new_node_population(id: u32, name: &str, related_constant_name: &str) -> Box<Node>;
        pub fn new_node_combinator(id: u32, name: &str, operation: u32) -> Box<Node>;

        pub fn model_from_json(json: &[u8]) -> Box<Model>;
        pub fn model_into_json(model: Box<Model>) -> String;

    }
}

pub fn new_model(start_time: f64, end_time: f64, delta_time: f64) -> Box<Model> {
    let meta_data = MetaData { start_time, end_time, delta_time };
    Box::new(Model {
        meta_data,
        nodes: HashMap::new(),
        constants: Vec::new(),
    })
}
pub fn new_node_population(id: u32, name: &str, related_constant_name: &str) -> Box<Node> {
 Box::new(Node::Population {
        id,
        name: name.into(),
        related_constant_name: related_constant_name.into(),
        links: Vec::new(),
    })
}
pub fn new_node_combinator(id: u32, name: &str, operation: u32) -> Box<Node> {
    Box::new(Node::Combinator {
        id,
        name: name.into(),
        operation: u32_to_char(operation),
        inputs: Vec::new(),
    })
}

impl Node {
    pub fn add_link(self: &mut Node, sign: u32, node_id: u32) {
        let link = Link { sign: u32_to_char(sign), node_id };
        match self {
            Node::Population { links, .. } => {
                links.push(link);
            }
            Node::Combinator { .. } => {
                panic!("Can't add a link to a Combinator");
            }
        }
    }
    pub fn add_input(&mut self, input: NodeId) {
        match self {
            Node::Population { .. } => {
                panic!("Can't add an input to a Population");
            }
            Node::Combinator { inputs, .. } => {
                inputs.push(input);
            }
        }
    }
}

impl Model {
    pub fn add_node(&mut self, node: Box<Node>) {
        let node = *node;
        match node {
            Node::Population { id, .. } => {
                self.nodes.insert(id, node);
            }
            Node::Combinator { id, .. } => {
                self.nodes.insert(id, node);
            }
        }
    }
    pub fn add_constant(self: &mut Model, name: &str, value: f64) {
        let constant = Constant { name: name.to_string(), value };
        self.constants.push(constant);
    }
    pub fn meta_data(&self) -> &MetaData {
        &self.meta_data
    }
    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
    pub fn get_node(&self, id: u32) -> OptionPtr<Node> {
        option_to_ptr(self.nodes.get(&id))
    }
}

impl MetaData {
    pub fn start_time(&self) -> f64 {
        self.start_time
    }
    pub fn end_time(&self) -> f64 {
        self.end_time
    }
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}

impl Node {
    pub fn id(&self) -> &u32 {
        match self {
            Node::Population { id, .. } => id,
            Node::Combinator { id, .. } => id,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Node::Population { name, .. } => &name,
            Node::Combinator { name, .. } => &name,
        }
    }
    pub fn related_constant_name(&self) -> &str {
        match self {
            Node::Population { related_constant_name, .. } => &related_constant_name,
            Node::Combinator { .. } => "",
        }
    }
    pub fn operation(&self) -> OptionPtr<u32> {
        match self {
            Node::Population { .. } => none_ptr(),
            // Safety: this is not unsafe at all. Every char is a valid u32, but not the other way
            // around
            Node::Combinator { operation, .. } => some_ptr(unsafe { std::mem::transmute(operation) }),
        }
    }
}
impl Link {
    pub fn sign(&self) -> u32 {
        self.sign as u32
    }
    pub fn node_id(&self) -> u32 {
        self.node_id as u32
    }
}

impl Constant {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}


pub fn u32_to_char(u: u32) -> char {
    char::from_u32(u).expect("Invalid char")
}

pub fn model_from_json(json: &[u8]) -> Box<Model> {
    Box::new(serde_json::from_slice(json).unwrap())
}

pub fn model_into_json(model: Box<Model>) -> String {
    serde_json::to_string(&*model).unwrap()
}
