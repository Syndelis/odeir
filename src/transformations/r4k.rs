use minijinja::{context, Environment};

use crate::{models::ode::OdeModel, Map};

const ODE_TEMPLATE: &str = include_str!("../../templates/ode.py.jinja");

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

    env.render_str(ODE_TEMPLATE, &mut ctx).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::Equation;
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
        Component {
            name: name.into(),
            contribution,
        }
    }

    /// Shorthand for positive arguments
    fn arg(name: impl Into<String>) -> Component {
        argument(name, '+')
    }

    fn equation(name: impl Into<String>, operates_on: impl Into<String>, composition: Component) -> Equation {
        Equation {
            name: name.into(),
            operates_on: Some(operates_on.into()),
            argument: composition.name,
            contribution: composition.contribution,
        }
    }

    #[test]
    fn render_simple() {
        let mut model = OdeModel::new("_".into(), Metadata {
            start_time: 10.0,
            ..Default::default()
        });

        model.insert_argument(value("A", 10_f64));
        model.insert_argument(value("B", 20_f64));
        model.insert_argument(value("k", 0.5));

        model.insert_argument(composite("A+B", "+", [arg("A"), arg("B")]));
        model.insert_equation(equation("dA/dt", "A", arg("A+B")));

        model.insert_argument(composite("(A+B)*k", "*", [arg("A+B"), arg("k")]));
        model.insert_equation(equation("dB/dt", "B", arg("(A+B)*k")));

        let ode = render_ode(&model);

        println!("{ode}");

        let expected = r#"import argparse, contextlib, sys, os
import scipy
import numpy as np

def initial_values() -> np.ndarray:
    A_0 = 10.0
    B_0 = 20.0
    return np.array((
        A_0,
        B_0,
        ))


def constants() -> list:
    k = 0.5
    return [
        k,
        ]


def system(t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    # populations
    A,B, = y

    # constants
    k, = constants
    
    dA_dt = A + B 
    dB_dt = (A + B ) * k 

    return np.array([dA_dt,dB_dt])"#;

        assert_eq!(ode, expected);
    }
}
