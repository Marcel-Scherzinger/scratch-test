#[derive(Debug, derive_more::Display)]
pub enum ExercisePart {
    #[display("a")]
    A,
    #[display("b")]
    B,
    #[display("c")]
    C,
}
use testreports::*;

pub const WARN_COUNTER_LOOP: Message<TestReport> =
    testreports::Message::cwarning("you shouldn't use counter-based loops");
pub const HINT_NO_EXTRA_SPACE: Message<TestReport> =
    Message::chint("your output doesn't separate the result with a space from the prefix text");

pub trait ExerciseTest {
    fn exercise(&self) -> (u8, ExercisePart);
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport;
}
