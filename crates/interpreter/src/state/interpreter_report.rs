use std::rc::Rc;

use crate::{ActionEntry, Interpreter, OutputAction, RResult, RunError, State};

#[derive(Debug, Clone)]
pub struct InterpreterReport {
    state: Rc<State>,
    run_error: Option<RunError>,
}

impl InterpreterReport {
    pub(crate) fn new(state: State, exit_status: Result<(), RunError>) -> Self {
        Self {
            state: state.into(),
            run_error: exit_status.err(),
        }
    }
}

impl InterpreterReport {
    pub fn all_output_actions(&self) -> impl Iterator<Item = (&OutputAction, &Rc<str>)> {
        self.state.actions.iter().flat_map(|a| {
            if let ActionEntry::Output { kind, msg } = a {
                Some((kind, msg))
            } else {
                None
            }
        })
    }
    pub fn all_output_texts(&self) -> impl Iterator<Item = &Rc<str>> {
        self.all_output_actions().map(|(_, t)| t)
    }

    pub fn predefined_answers(&self) -> &Rc<[model::VariableValue]> {
        &self.state.predefined_answers
    }
    pub fn warn_used_counter_loop(&self) -> bool {
        *self.state.warnings.used_counter_loop()
    }
    pub fn run_error(&self) -> Option<&RunError> {
        self.run_error.as_ref()
    }
}
