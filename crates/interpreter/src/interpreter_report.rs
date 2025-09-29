use crate::{Finished, Interpreter, OutputAction};

impl Interpreter<Finished> {
    pub fn all_output_actions(&self) -> impl Iterator<Item = (&OutputAction, &String)> {
        self.state.all_output_actions()
    }
}
