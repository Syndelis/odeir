use crate::Node;

pub mod ode;
pub mod r4k;

impl crate::Model {
    fn get_nodes_of<'a>(&'a self, mut cb: impl FnMut(&'a Node)->bool) -> impl Iterator<Item=&'a Node> {
        self.nodes
            .iter()
            .map(|(_, node)| node)
            .filter(move |node| cb(node))
    }
    pub fn get_populations<'a>(&'a self) -> impl Iterator<Item=&'a Node> {
        self.get_nodes_of(|node| matches!(node, &Node::Population{..}))
    }
    pub fn get_constants<'a>(&'a self) -> impl Iterator<Item=&'a Node> {
        self.get_nodes_of(|node| matches!(node, &Node::Constant{..}))
    }
    pub fn get_combinators<'a>(&'a self) -> impl Iterator<Item=&'a Node> {
        self.get_nodes_of(|node| matches!(node, &Node::Combinator{..}))
    }
}
