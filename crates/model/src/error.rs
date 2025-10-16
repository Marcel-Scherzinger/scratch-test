use std::path::PathBuf;

pub use crate::ext::JsonCtxError;
pub use crate::interpret_json::FormatError;
pub use crate::scopes::error::{
    TargetBlocksError, TargetError, TargetListsError, TargetVariablesError,
};
pub use crate::scratch_expr::IntegerOutOfBounds;

#[allow(unused)]
pub use crate::blocks::{BlockAttrError, BlockKindError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("target error: {0}")]
    Target(#[from] JsonCtxError<TargetError>),

    /// The saved variable values are provided in an invalid way
    #[error("error in target variables {0}")]
    TargetVar(#[from] JsonCtxError<TargetVariablesError>),
    /// The saved variable values are provided in an invalid way
    #[error("error in target lists {0}")]
    TargetLis(#[from] JsonCtxError<TargetListsError>),
    /// The target blocks (the most important compononet) is invalid
    #[error("{0}")]
    TargetBlocks(Box<TargetBlocksError>),
}
impl From<TargetBlocksError> for Error {
    fn from(value: TargetBlocksError) -> Self {
        Self::TargetBlocks(value.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error("model: {0}")]
    Model(#[from] Error),

    #[error("stream or file was used that doesn't contain a document")]
    NoDocument,
    #[error("io error: {0}")]
    Io(#[from] Box<dyn std::error::Error>),
    #[error("invalid json document: {0}")]
    Json(#[from] serde_json::Error),
    #[error("file {0:?} read error: {1}")]
    FileRead(PathBuf, std::io::Error),
}
