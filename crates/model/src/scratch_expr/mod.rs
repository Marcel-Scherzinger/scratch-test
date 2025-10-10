mod svalue;

use std::borrow::Cow;

pub trait ScratchExpr {
    fn as_text(&self) -> Cow<'_, str>;
    fn as_int(&self) -> i64;
    fn as_float(&self) -> f64;
}
#[derive(Debug, thiserror::Error, PartialEq)]
#[error("integer out of bounds")]
pub struct IntegerOutOfBounds;

pub use svalue::SValue;
