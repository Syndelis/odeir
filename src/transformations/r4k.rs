use minijinja::{context, Environment};

use crate::{models::ode::OdeModel, Map};

const ODE_TEMPLATE: &str = include_str!("../../templates/ode.py.txt");

pub fn render_ode(model: &OdeModel) -> String {
    let env = Environment::new();

    let populations = model.get_populations().collect::<Vec<_>>();
    let constants = model.get_constants().collect::<Vec<_>>();
    let equations = model.equations.iter().cloned().filter_map(|eq| Some((eq.operates_on.clone()?, eq))).collect::<Map<_, _>>();

    let mut ctx = context! {
        model => model,
        equations => equations,
        populations => populations,
        constants => constants,
    };

    dbg!(&ctx);

    env.render_str(ODE_TEMPLATE, &mut ctx).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::models::ode::Metadata;
    use crate::models::{Argument, Component};

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
        let mut model = Model::new("_".into(), Metadata {
            start_time: 10.0,
            ..Default::default()
        });
        model.insert_argument(value("w", 9.0));
        model.insert_argument(value("x", -1.0));
        model.insert_argument(value("y", 10.0));
        // dx
        model.insert_argument(composite("xy", "*", [arg("y"), arg('x')]));

        model.insert_argument(composite("sub", "-", [arg("w"), arg('x')]));

        // model.equations.insert_equation("x", "xy");
        // model.equations.insert_equation("y", "sub");

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
