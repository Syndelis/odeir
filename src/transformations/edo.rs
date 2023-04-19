use minijinja::{context, Environment};

use crate::Model;

const EDO_TEMPLATE: &str = include_str!("../../templates/edo.txt");

pub fn render_edo(model: Model) -> String {

    let mut env = Environment::new();

    let mut ctx = context! {
        model => model,
    };

    env.render_str(EDO_TEMPLATE, &mut ctx).unwrap()

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn test_render_edo_abc_json() {
        use super::*;

        const ABC_JSON_STR: &str = include_str!("../../tests/fixtures/abc.json");

        let model = serde_json::from_str::<Model>(ABC_JSON_STR).unwrap();

        let edo = render_edo(model);

        const EXPECTED: &str = "Equations:

        - dA/dt =+ (A  * B ) +
        - dB/dt =- (A  * B ) +
        - dC/dt =+ (A  * B  / C ) +";

        assert_eq!(edo, EXPECTED);
    }
}
