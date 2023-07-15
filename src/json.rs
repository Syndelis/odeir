use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Node, MetaData, Link, NodeId, Operation, Model};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonModel {
    meta_data: MetaData,
    links: Vec<Link>,
    nodes: Vec<JsonNode>
}

impl From<JsonModel> for Model {
    fn from(value: JsonModel) -> Self {
        let JsonModel { meta_data, links, nodes } = value;
        let mut nodes: HashMap<NodeId, Node> = nodes.into_iter()
            .map(|jnode| {
                let JsonNode {id, name, node_info} = jnode;
                match node_info {
                    JsonNodeInfo::Constant { value } => Node::Constant { id, name, outputs: vec![], value },
                    JsonNodeInfo::Population { initial_population } => Node::Population { id, name, outputs: vec![], initial_population },
                    JsonNodeInfo::Combinator { operation } =>          Node::Combinator { id, name, outputs: vec![], inputs: vec![], operation},
                }
            })
            .map(|node| (node.id(), node)).collect();
        for link in &links {
            if let Node::Combinator {inputs, ..} = nodes.get_mut(&link.receiver).unwrap() {
                inputs.push(link.clone());
            } else {
                // !TODO: turn this into an error instead of panic
                panic!("Link receiver is not a combinator")
            }
            nodes.get_mut(&link.sender).unwrap().outputs().push(link.clone());
        }
        Self {
            links,
            meta_data,
            nodes,
        }
    }
}

impl From<Model> for JsonModel {
    fn from(value: Model) -> Self {
        let Model { meta_data, links, nodes } = value;
        let mut nodes: Vec<JsonNode> = nodes.into_values().map(|node| match node {
            Node::Constant { id, name, value, .. } => JsonNode { id, name, node_info: JsonNodeInfo::Constant { value }},
            Node::Population { id, name, initial_population, .. } => JsonNode { id, name, node_info: JsonNodeInfo::Population { initial_population }},
            Node::Combinator { id, name, operation, .. } => JsonNode { id, name, node_info: JsonNodeInfo::Combinator { operation }},
        }).collect();
        // Remember to sort the nodes by id before serializing
        // Not that this is important to the json, but it makes it easier to diff
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        Self {
            meta_data,links,nodes
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum JsonNodeInfo {
    Constant {value: f64},
    Population {initial_population: f64},
    Combinator {operation: Operation},
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonNode {
    id: NodeId,
    name: String,
    #[serde(flatten)]
    node_info: JsonNodeInfo,
}
