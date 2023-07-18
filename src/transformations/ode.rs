use minijinja::{context, Environment};

use crate::{Model, Node};

const ODE_TEMPLATE: &str = include_str!("../../templates/ode.txt");

pub fn render_ode(model: Model) -> String {
    let env = Environment::new();

    let mut ctx = context! {
        model => model,
    };

    env.render_str(ODE_TEMPLATE, &mut ctx).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_render_ode_abc_json() {
        use super::*;

        const ABC_JSON_STR: &str = include_str!("../../tests/fixtures/abc.json");

        let model = serde_json::from_str::<Model>(ABC_JSON_STR).unwrap();

        let ode = render_ode(model);

        const EXPECTED: &str = "Equations:

        - dA/dt =+ (A  * B ) +
        - dB/dt =- (A  * B ) +
        - dC/dt =+ (A  * B  / C ) +";

        assert_eq!(ode, EXPECTED);
    }
}
