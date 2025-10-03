use std::{collections::HashMap, rc::Rc};

use crate::{Messages, OutputListComparison, ProgramError, Text};

#[derive(Debug, Clone, PartialEq)]
pub struct TestCase {
    pub(crate) messages: Messages<TestCase>,
    pub(crate) expected_output: Option<Rc<[model::VariableValue]>>,
    #[allow(unused)]
    pub(crate) data_lists: HashMap<Text, OutputListComparison<Text>>,
    pub(crate) interpreter: interpreter::InterpreterReport,
}
impl TestCase {
    pub fn out(&self) -> &interpreter::InterpreterReport {
        &self.interpreter
    }
    pub fn set_expected_output<T: Into<model::VariableValue>>(
        &mut self,
        eo: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        self.expected_output = Some(eo.into_iter().map(|s| s.into()).collect());
        self
    }
    pub fn expected_output(&self) -> &Option<Rc<[model::VariableValue]>> {
        &self.expected_output
    }
    pub fn local_messages(&self) -> &Messages<TestCase> {
        &self.messages
    }
    pub fn program_error(&self) -> Option<ProgramError> {
        use interpreter::RunError as E;
        Some(match self.interpreter.run_error()? {
            E::WaitTillNeverStops | E::ConditionLoopWithoutBodyNeverStops => {
                // program uses wait till block with condition = true
                ProgramError::DoesntTerminate
            }
            E::AccessUnknownList(_)
            | E::AccessUnknownVariable(_)
            | E::PopOnEmptyProgramStack
            | E::ReachedUnknownBlock(_)
            | E::UnexpectedNestingOfBlocks
            | E::UnsupportedMathOperator(_)
            | E::UnexpectedBlockKind(_) => {
                // log::error!("Scratch error: {err} ({err:?})");
                ProgramError::ScratchInteractionError
            }
            E::AllowedNumberOfExecutedStmtsExceeded => ProgramError::ExecutedTooManyBlocks,
            E::TerminateBecauseOfStop => {
                return None;
            }
            E::QuestionAskedWithoutAnswer => ProgramError::QuestionWithoutAnswer,
        })
    }
}
