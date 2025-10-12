use std::fmt::Display;

use derive_more::{Debug, Display, From};

#[derive(Debug, Display, From)]
#[debug("{_0:?}")]
#[display("{_0}")]
pub struct CmpUsingDisplay<T>(T);

impl<T> PartialEq for CmpUsingDisplay<T>
where
    T: Display,
{
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
