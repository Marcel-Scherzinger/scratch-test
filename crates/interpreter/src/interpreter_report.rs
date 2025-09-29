use crate::{Finished, Interpreter, OutputAction, RResult};

impl Interpreter<Finished> {
    pub fn all_output_actions(&self) -> impl Iterator<Item = (&OutputAction, &String)> {
        self.state.all_output_actions()
    }
    pub fn warn_used_counter_loop(&self) -> bool {
        *self.state.warnings().used_counter_loop()
    }
    pub fn result(&self) -> &RResult<()> {
        &self.result
    }
}
