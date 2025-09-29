mod a1;
pub(crate) mod defs;

pub use a1::{A1a, A1b};
pub use defs::{ExercisePart, ExerciseTest, FailedTestRun, ProgramError, TestReport, Warning};

pub fn exercises(number: u8) -> Option<std::rc::Rc<[Box<dyn ExerciseTest>]>> {
    use std::rc::Rc;
    Some(match number {
        1 => Rc::new([Box::new(A1a), Box::new(A1b)]),
        _ => return None,
    })
}
