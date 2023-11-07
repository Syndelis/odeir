use serde::{Deserialize, Serialize};

use crate::{
    models::{self, Argument, CoreModel, Equation},
    Map,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Json {
    pub metadata: Metadata,
    pub arguments: Vec<Argument>,
    pub equations: Vec<Equation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ModelMetadata {
    #[serde(rename = "ode")]
    ODE(models::ode::Metadata),
    #[serde(rename = "cellular-automata")]
    CellularAutomata {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    #[serde(default)]
    pub name: String,
    #[serde(flatten)]
    pub model_metadata: ModelMetadata,
    #[serde(default)]
    pub positions: Map<String, Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "Json")]
#[serde(into = "Json")]
pub enum Model {
    ODE(models::ode::Model),
    CellularAutomata(models::cellular_automata::Model),
}

impl From<Json> for Model {
    fn from(value: Json) -> Self {
        let core = CoreModel {
            arguments: value
                .arguments
                .into_iter()
                .map(|arg| (arg.name().to_owned(), arg))
                .collect(),
            equations: value.equations,
        };
        match value.metadata.model_metadata {
            ModelMetadata::CellularAutomata {} => {
                Self::CellularAutomata(models::cellular_automata::Model { core })
            }
            ModelMetadata::ODE(metadata) => Self::ODE(models::ode::Model { core, metadata }),
        }
    }
}

impl From<Model> for Json {
    fn from(value: Model) -> Self {
        match value {
            Model::ODE(model) => Json {
                metadata: Metadata {
                    name: "TODO".into(),
                    model_metadata: ModelMetadata::ODE(model.metadata),
                    positions: Map::new(),
                },
                arguments: model.core.arguments.into_values().collect(),
                equations: model.core.equations,
            },
            Model::CellularAutomata(_) => todo!("Implement CA"),
        }
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;

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
            equations: [
                Equation {
                    name: "dead_equation".to_string(),
                    operates_on: "dead".to_string(),
                    argument: "dead_equation".to_string(),
                    contribution: '+',
                },
                Equation {
                    name: "alive_equation".to_string(),
                    operates_on: "alive".to_string(),
                    argument: "alive_equation".to_string(),
                    contribution: '+',
                },
            ]
            .to_vec(),
        };
        assert_json_eq!(expected, model);
    }
}
