mod case;
mod category;
mod category_tests;
mod message;
mod report;

pub use case::TestCase;
pub use category::Category;
pub use category_tests::CategoryTests;
pub use message::{Message, MessageAdder, MessageHub, MessageKind};
pub use report::TestReport;

use std::rc::Rc;

pub use crate::message::Messages;
type Text = Rc<str>;

#[derive(Debug, PartialEq, Clone)]
pub struct OutputListComparison {
    program: Rc<[model::SValue]>,
    expected: Rc<[model::SValue]>,
}

impl OutputListComparison {
    pub fn program(&self) -> &Rc<[model::SValue]> {
        &self.program
    }
    pub fn expected(&self) -> &Rc<[model::SValue]> {
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
    #[display("program requested a random number range not matching the provided random number")]
    InvalidRandomRequest,
}
