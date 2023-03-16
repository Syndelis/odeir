use minijinja::{context, Environment};

use crate::rustside::Model;

const EDO_TEMPLATE: &str = include_str!("../../templates/edo.txt");

pub fn render_edo(model: Model) -> String {

    let mut env = Environment::new();

    let mut ctx = context! {
        model => model,
    };

    env.render_str(EDO_TEMPLATE, &mut ctx).unwrap()

}
