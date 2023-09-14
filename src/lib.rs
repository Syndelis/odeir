#![feature(vec_into_raw_parts)]
pub mod ffi;
pub mod transformations;

use std::collections::BTreeMap as Map;

use serde::{Deserialize, Serialize};

pub type NodeId = u32;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(try_from = "char")]
#[serde(into = "char")]
#[repr(u8)]
pub enum LinkType {
    #[default]
    Normal,
    Negative,
}

impl Into<char> for LinkType {
    fn into(self) -> char {
        match self {
            Self::Normal => '+',
            Self::Negative => '-',
        }
    }
}
impl TryFrom<char> for LinkType {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '+' => Self::Normal,
            '-' => Self::Negative,
            _ => return Err(format!("Expected either + or -"))
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Link {
    pub sender: NodeId,
    #[serde(default)]
    pub link_type: LinkType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Model {
    pub meta_data: MetaData,
    pub nodes: Map<NodeId, Node>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MetaData {
    pub start_time: f64,
    pub end_time: f64,
    pub delta_time: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
    Constant {
        id: NodeId,
        name: String,

        value: f64,
    },
    Population {
        id: NodeId,
        name: String,

        initial_population: f64,
    },
    Combinator {
        id: NodeId,
        name: String,

        operation: Operation,
        inputs: Vec<Link>,
    },
    Assigner {
        id: NodeId,

        population: NodeId,
        inputs: Vec<Link>,
    }
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Self::Constant { name, .. } => name,
            Self::Population { name, .. } => name,
            Self::Combinator { name, .. } => name,
            Self::Assigner { .. } => "Assigner",
        }
    }
    pub fn id(&self) -> NodeId {
        match self {
            Self::Constant { id, .. } => *id,
            Self::Population { id, .. } => *id,
            Self::Combinator { id, .. } => *id,
            Self::Assigner { id, .. } => *id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(try_from = "char")]
#[serde(into = "char")]
#[repr(u8)]
pub enum Operation {
    #[default]
    Add,
    Sub,
    Div,
    Mul,
}

impl Into<char> for Operation {
    fn into(self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Div => '/',
            Self::Mul => '*',
        }
    }
}
impl TryFrom<char> for Operation {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '/' => Ok(Self::Div),
            '*' => Ok(Self::Mul),
            _ => Err(format!("Expected either '+', '-', '/', or '*'")),
        }
    }
}

pub fn model_into_json(model: &Model) -> String {
    serde_json::to_string(model).unwrap()
}
