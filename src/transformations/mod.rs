use crate::models::Argument;

// pub mod ode;
pub mod r4k;

impl crate::models::Equations {
    fn get_equations_of<'a>(
        &'a self,
        mut cb: impl FnMut(&'a Argument) -> bool,
    ) -> impl Iterator<Item = &'a Argument> {
        self.arguments
            .iter()
            .map(|(_, node)| node)
            .filter(move |node| cb(node))
    }
    pub fn get_populations<'a>(&'a self) -> impl Iterator<Item = &'a Argument> {
        self.get_equations_of(|arg| match arg {
            Argument::Value { name, .. } => self.equations.contains_key(name),
            _ => false,
        })
    }
    pub fn get_constants<'a>(&'a self) -> impl Iterator<Item = &'a Argument> {
        self.get_equations_of(|arg| match arg {
            Argument::Value { name, .. } => !self.equations.contains_key(name),
            _ => false,
        })
    }
}
