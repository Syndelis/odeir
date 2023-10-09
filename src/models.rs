use serde::{Deserialize, Serialize};

use crate::Map;

pub mod cellular_automata;
pub mod ode;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Equations {
    pub arguments: Map<String, Argument>,
    pub equations: Map<String, String>,
}

impl Equations {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn insert_argument(&mut self, arg: Argument) {
        self.arguments.insert(arg.name().to_owned(), arg);
    }
    pub fn insert_equation(&mut self, argument_name: impl Into<String>, equation_name: impl Into<String>) {
        self.equations.insert(argument_name.into(), equation_name.into());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Argument {
    Value {
        name: String,
        value: f64,
    },
    Composite {
        name: String,
        operation: String,
        composition: Vec<Component>,
    },
}

impl Argument {
    pub fn name(&self) -> &str {
        match self {
            Argument::Value { name, .. } => name,
            Argument::Composite { name, .. } => name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Component {
    Argument { name: String, contribution: char },
    Constant { value: f64, contribution: char },
}

impl Component {
    fn contribution(&self) -> char {
        match self {
            Component::Argument { contribution, .. } | Component::Constant { contribution, .. } => {
                *contribution
            }
        }
    }
}
