mod case;
mod category;
mod category_tests;
mod message;
mod report;

pub use case::TestCase;
pub use category::Category;
pub use category_tests::CategoryTests;
pub use message::{Message, MessageKind};
pub use report::TestReport;

use std::rc::Rc;

pub use crate::message::Messages;
type Text = Rc<str>;

#[derive(Debug, PartialEq, Clone)]
pub struct OutputListComparison {
    program: Rc<[model::VariableValue]>,
    expected: Rc<[model::VariableValue]>,
}

impl OutputListComparison {
    pub fn program(&self) -> &Rc<[model::VariableValue]> {
        &self.program
    }
    pub fn expected(&self) -> &Rc<[model::VariableValue]> {
        &self.expected
    }
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
