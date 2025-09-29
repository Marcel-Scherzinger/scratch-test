#![allow(unused)]
use std::collections::BTreeSet;

#[derive(Debug, derive_more::Display)]
pub enum ExercisePart {
    #[display("a")]
    A,
    #[display("b")]
    B,
    #[display("c")]
    C,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Warning {
    CounterLoop,
    NoExtraSpace,
}

impl Warning {
    pub fn en_msg(&self) -> String {
        match self {
            Self::CounterLoop => "used counter based loop".to_string(),
            Self::NoExtraSpace => {
                "output doesn't separate prefix and result with space".to_string()
            }
        }
    }
}

#[derive(Debug, derive_getters::Getters)]
pub struct FailedTestRun {
    pub(crate) inputs: Vec<String>,
    pub(crate) program_output: Vec<String>,
    pub(crate) expected_output: Vec<String>,
    pub(crate) exit_status: Option<ProgramError>,
}

#[derive(Debug, derive_getters::Getters)]
pub struct TestReport {
    pub(crate) perfect_cases: usize,
    pub(crate) error_cases: Vec<FailedTestRun>,
    pub(crate) warnings: BTreeSet<Warning>,
}

pub trait ExerciseTest {
    fn exercise(&self) -> (u8, ExercisePart);
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport;
}
#[derive(Debug, Clone, derive_more::Display)]
pub enum ProgramError {
    #[display("doesn't terminate (within set limits)")]
    DoesntTerminate,
    #[display("error with the scratch file")]
    ScratchInteractionError,
    #[display("program executed more than allowed blocks")]
    ExecutedTooManyBlocks,
    #[display("program asked too many questions")]
    QuestionWithoutAnswer,
}

pub(crate) fn deal_with_run_error(err: &interpreter::RunError) -> Option<ProgramError> {
    use interpreter::RunError as E;
    Some(match err {
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
            log::error!("Scratch error: {err} ({err:?})");
            ProgramError::ScratchInteractionError
        }
        E::AllowedNumberOfExecutedStmtsExceeded => ProgramError::ExecutedTooManyBlocks,
        E::TerminateBecauseOfStop => {
            return None;
        }
        E::QuestionAskedWithoutAnswer => ProgramError::QuestionWithoutAnswer,
    })
}
