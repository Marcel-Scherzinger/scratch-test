mod a1;
pub(crate) mod defs;

pub use a1::{A1a, A1b};
pub use defs::{ExercisePart, ExerciseTest, FailedTestRun, ProgramError, TestReport, Warning};

pub fn exercises(number: u8) -> Option<std::rc::Rc<[Option<std::rc::Rc<dyn ExerciseTest>>]>> {
    use std::rc::Rc;
    Some(match number {
        1 => Rc::new([Some(Rc::new(A1a)), Some(Rc::new(A1b))]),
        _ => return None,
    })
}
