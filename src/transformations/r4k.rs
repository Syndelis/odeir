use minijinja::{context, Environment};

use crate::models::{ode::Model, Argument, Component, Equations};

const ODE_TEMPLATE: &str = include_str!("../../templates/ode.py.txt");

pub fn render_ode(model: &Model) -> String {
    let env = Environment::new();

    let populations = model.equations.get_populations().collect::<Vec<_>>();
    let constants = model.equations.get_constants().collect::<Vec<_>>();

    let mut ctx = context! {
        model => model,
        populations => populations,
        constants => constants,
    };

    env.render_str(ODE_TEMPLATE, &mut ctx).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::models::ode::Metadata;

    use super::*;

    fn value(name: impl Into<String>, value: f64) -> Argument {
        Argument::Value {
            name: name.into(),
            value,
        }
    }

    fn composite(
        name: impl Into<String>,
        operation: impl Into<String>,
        composition: impl IntoIterator<Item = Component>,
    ) -> Argument {
        Argument::Composite {
            name: name.into(),
            operation: operation.into(),
            composition: composition.into_iter().collect(),
        }
    }

    fn argument(name: impl Into<String>, contribution: char) -> Component {
        Component::Argument {
            name: name.into(),
            contribution,
        }
    }

    /// Shorthand for positive arguments
    fn arg(name: impl Into<String>) -> Component {
        argument(name, '+')
    }

    fn constant(value: f64, contribution: char) -> Component {
        Component::Constant {
            value,
            contribution,
        }
    }

    #[test]
    fn render_simple() {
        let mut model = Model::new(Metadata { start_time: 10.0, ..Default::default()});
        model.equations.insert_argument(value("w", 9.0));
        model.equations.insert_argument(value("x", -1.0));
        model.equations.insert_argument(value("y", 10.0));
        // dx
        model.equations.insert_argument(composite(
            "xy",
            "*",
            [arg("y"), arg('x')],
        ));

        model.equations.insert_argument(composite(
            "sub",
            "-",
            [arg("w"), arg('x')],
        ));

        model.equations.insert_equation("x", "xy");
        model.equations.insert_equation("y", "sub");

        let ode = render_ode(&model);
        let expected = r#"import numpy as np
def system( t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    # populations
    x,y, = y

    # constants
    w, = constants
    
    dx = y * x 
    dy = w - x 

    return np.array([dx,dy])"#;
        assert_eq!(ode, expected);
    }
}
