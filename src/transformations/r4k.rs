use std::collections::HashSet;

use minijinja::{context, Environment};

use crate::{Model, Node};

const ODE_TEMPLATE: &str = include_str!("../../templates/ode.py.txt");

pub fn render_ode(model: &Model) -> String {
    let env = Environment::new();

    fn get_nodes_of(model: &Model, cb: impl Fn(&Node)->bool) -> Vec<&Node> {
        let mut tmp = model.nodes
			 .iter()
			 .filter(|(_, node)| cb(node)).collect::<Vec<_>>();
        tmp.sort_by_cached_key(|(k, _)| *k);
        tmp.into_iter()
			 .map(|(_, node)| node)
			 .collect()
    }

    let constants = get_nodes_of(&model, |node| matches!(node, Node::Constant{..}));
    let populations = get_nodes_of(&model, |node| matches!(node, Node::Population{..}));

    let mut ctx = context! {
        model => model,
        constants => constants,
        populations => populations,
    };

    env.render_str(ODE_TEMPLATE, &mut ctx).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_render_r4k_abc_json() {
        use super::*;

        const ABC_JSON_STR: &str = include_str!("../../tests/fixtures/abc.json");

        let model = serde_json::from_str::<Model>(ABC_JSON_STR).unwrap();

        let ode = render_ode(&model);

        std::fs::write("/tmp/ode.py", &ode).unwrap();

        const EXPECTED: &str = r#"import numpy as np
def system( t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    A,B,C, = y

     = constants


    dA = +(A  * B )
    dB = -(A  * B )
    dC = +(A  * B  / C )

    return np.array([dA, dB, dC, ])"#;

        assert_eq!(ode, EXPECTED);
    }
}
