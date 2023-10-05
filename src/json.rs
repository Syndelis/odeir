use assert_json_diff::assert_json_eq;
use serde::{Deserialize, Serialize};

use crate::{
    models::{self, Argument, Equations},
    Map,
};

#[derive(Serialize, Deserialize, Debug)]
struct Json {
    metadata: Metadata,
    arguments: Vec<Argument>,
    equations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ModelMetadata {
    #[serde(rename = "ode")]
    ODE(models::ode::Metadata),
    #[serde(rename = "cellular-automata")]
    CellularAutomata {},
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    #[serde(flatten)]
    model_metadata: ModelMetadata,
    positions: Map<String, Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub enum Model {
    ODE(models::ode::Model),
    CellularAutomata(models::cellular_automata::Model),
}

impl From<Json> for Model {
    fn from(value: Json) -> Self {
        let equations = Equations {
            arguments: value
                .arguments
                .into_iter()
                .map(|arg| (arg.name().to_owned(), arg))
                .collect(),
            equations: value.equations,
        };
        match value.metadata.model_metadata {
            ModelMetadata::CellularAutomata {} => {
                Self::CellularAutomata(models::cellular_automata::Model { equations })
            }
            ModelMetadata::ODE(metadata) => Self::ODE(models::ode::Model {
                equations,
                metadata,
            }),
        }
    }
}

impl From<Model> for Json {
    fn from(value: Model) -> Self {
        todo!("serialization")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::models::Component;

    #[test]
    fn deserialize_game_of_life() {
        let file = include_str!("../fixtures/game-of-life.json");
        let model = serde_json::from_str::<Json>(file).unwrap();
        let expected = Json {
            metadata: Metadata {
                name: "Conway's Game of Life".into(),
                model_metadata: ModelMetadata::CellularAutomata {},
                positions: Map::new(),
            },
            arguments: vec![
                Argument::Value {
                    name: "dead".into(),
                    value: 0.5,
                },
                Argument::Value {
                    name: "alive".into(),
                    value: 0.5,
                },
                Argument::Composite {
                    name: "reproduction".into(),
                    operation: "==".into(),
                    composition: vec![
                        Component::Argument {
                            name: "alive".into(),
                            contribution: '+',
                        },
                        Component::Constant {
                            value: 3.0,
                            contribution: '+',
                        },
                    ],
                },
                Argument::Composite {
                    name: "overpopulation".into(),
                    operation: ">".into(),
                    composition: vec![
                        Component::Argument {
                            name: "alive".into(),
                            contribution: '+',
                        },
                        Component::Constant {
                            value: 3.0,
                            contribution: '+',
                        },
                    ],
                },
                Argument::Composite {
                    name: "underpopulation".into(),
                    operation: "<".into(),
                    composition: vec![
                        Component::Argument {
                            name: "alive".into(),
                            contribution: '+',
                        },
                        Component::Constant {
                            value: 2.0,
                            contribution: '+',
                        },
                    ],
                },
                Argument::Composite {
                    name: "dead_equation".into(),
                    operation: "+".into(),
                    composition: vec![
                        Component::Argument {
                            name: "overpopulation".into(),
                            contribution: '+',
                        },
                        Component::Argument {
                            name: "underpopulation".into(),
                            contribution: '+',
                        },
                        Component::Argument {
                            name: "reproduction".into(),
                            contribution: '-',
                        },
                    ],
                },
                Argument::Composite {
                    name: "alive_equation".into(),
                    operation: "-".into(),
                    composition: vec![
                        Component::Constant {
                            value: 1.0,
                            contribution: '+',
                        },
                        Component::Argument {
                            name: "dead_equation".into(),
                            contribution: '+',
                        },
                    ],
                },
            ],
            equations: vec!["dead_equation".into(), "alive_equation".into()],
        };
        assert_json_eq!(expected, model);
    }
}
