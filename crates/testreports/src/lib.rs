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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct OutputListComparison<T> {
    program: Rc<[T]>,
    expected: Rc<[T]>,
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
