use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;

use crate::{Message, Messages, OutputListComparison, ProgramError, TestReport, Text};

#[derive(Debug, Clone, PartialEq)]
pub struct TestCase {
    pub(crate) messages: Messages<TestCase>,
    pub(crate) expected_output: Option<Rc<[model::VariableValue]>>,
    #[allow(unused)]
    pub(crate) data_lists: HashMap<Text, OutputListComparison>,
    pub(crate) interpreter: Box<interpreter::InterpreterReport>,
}
impl TestCase {
    pub fn set_list_comparison<P, E>(
        &mut self,
        listname: impl Into<Text>,
        program: impl IntoIterator<Item = P>,
        expected: impl IntoIterator<Item = E>,
    ) -> &mut Self
    where
        P: Into<model::VariableValue>,
        E: Into<model::VariableValue>,
    {
        self.data_lists.insert(
            listname.into(),
            OutputListComparison {
                program: program.into_iter().map(|p| p.into()).collect(),
                expected: expected.into_iter().map(|e| e.into()).collect(),
            },
        );
        self
    }
    pub fn differing_list_values(&self) -> impl Iterator<Item = (&Text, &OutputListComparison)> {
        self.data_lists.iter()
    }
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
    pub fn get_required_list(
        &self,
        name: &str,
    ) -> Result<&[model::VariableValue], Message<TestReport>> {
        let mut candidates = self
            .out()
            .all_lists()
            .filter_map(|(_id, lname, _, content)| (name == lname).then_some(content))
            .peekable();
        if candidates.peek().is_none() {
            return Err(Message::warning(format!(
                "required list {name:?} is missing"
            )));
        }
        candidates
            .exactly_one()
            .map_err(|_| Message::warning(format!("list name {name:?} is not unique")))
    }
}
