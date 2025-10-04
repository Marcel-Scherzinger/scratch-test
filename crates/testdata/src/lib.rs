mod a1;
mod a2;
pub(crate) mod defs;

pub use a1::{A1a, A1b};
pub use a2::A2a;
pub use defs::{ExercisePart, ExerciseTest};
pub use testreports::ProgramError;

type MaybeTestRunner = Option<std::rc::Rc<dyn ExerciseTest>>;

pub fn exercises(number: u8) -> Option<std::rc::Rc<[MaybeTestRunner]>> {
    use std::rc::Rc;
    Some(match number {
        1 => Rc::new([Some(Rc::new(A1a)), Some(Rc::new(A1b))]),
        2 => Rc::new([Some(Rc::new(A2a))]),
        _ => return None,
    })
}
