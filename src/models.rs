use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Map, Position};

pub mod cellular_automata;
pub mod ode;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CoreModel {
    pub arguments: Map<String, Argument>,
    pub equations: Vec<Equation>,
    pub positions: Map<String, Position>,
}

impl CoreModel {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn insert_argument(&mut self, arg: Argument) {
        self.arguments.insert(arg.name().to_owned(), arg);
    }
    pub fn insert_equation(&mut self, eq: Equation) {
        self.equations.push(eq);
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
        #[serde(default)]
        style: CompositionStyle,
        composition: Vec<Component>,
    },
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum CompositionStyle {
    #[default]
    Infixed,
    Prefixed,
}

impl Argument {
    pub fn name(&self) -> &str {
        match self {
            Argument::Value { name, .. } => name,
            Argument::Composite { name, .. } => name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Component {
    pub name: String,
    pub contribution: char,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equation {
    pub name: String,
    pub operates_on: Option<String>,
    pub argument: String,
    pub contribution: char,
}

//ODE structure for the real time Rust solver 
#[derive(Debug, Clone)]
pub struct OdeEquation {
    name: String,
    text: String, //texto da equação a ser avaliado
    expressions: Vec<String>,
    value: f64,
}

impl OdeEquation {
    pub fn new(name: String, text: String) -> Self {
        Self {
            name: name,
            text: text,
            expressions: vec![],
            value: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OdeSystem {
    pub equations: BTreeMap<String,OdeEquation>,
    pub values: BTreeMap<String,f64>, 
}
//gera os índices das populações em ordem alfabética.

impl OdeSystem {
    pub fn new() -> Self{
        Self {
            equations: BTreeMap::new(),
            values: BTreeMap::new(),
        }
    }

    pub fn create_ode_system(input: String) -> OdeSystem{
        let mut ode_system = OdeSystem::new();

        let lines = input.trim().split("\n").collect::<Vec<_>>();
    
        for line in lines {   

            let new_line = line.split("=")
                        .filter(|&s| !s.is_empty())
                        .collect::<Vec<_>>();
            let population = new_line[0].trim();
            let equation = new_line[1].trim();
            
            let expressions = equation.split("+")
                        .filter(|&s| !s.is_empty())
                        .collect::<Vec<_>>();                    
    
            let mut ode = OdeEquation::new(population.to_string(), equation.to_string());
            ode.expressions = expressions.iter().map(|&s| s.to_string()).collect::<Vec<String>>();

            ode_system.equations.insert(population.to_string(), ode);
        }

        return ode_system
    }
}

