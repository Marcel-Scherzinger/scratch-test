use super::CmpUsingDisplay;
use crate::{error::IntegerOutOfBounds, interpret_json::OpcodeNum};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FormatError {
    /// A read number is too big
    #[error("{0}")]
    IntegerBounds(IntegerOutOfBounds),
    #[error("unable to parse number")]
    NoNumber,
    #[error("opcode={opcode} is not allowed to have {value:?} as value")]
    UnexpectedNumberKind {
        opcode: OpcodeNum,
        value: serde_json::Value,
    },
    #[error("number not in scope of expected type")]
    NumberOutOfRange,
    #[error("read opcode was null")]
    OpcodeNull,
    #[error("found opcode={0} is invalid at this point")]
    UnexpectedOpcode(OpcodeNum),

    #[error("missing variable name (text) at index={0}")]
    MissingVarName(u8),
    #[error("missing variable id (text) at index={0}")]
    MissingVarId(u8),

    #[error("missing list name (text) at index={0}")]
    MissingListName(u8),
    #[error("missing list id (text) at index={0}")]
    MissingListId(u8),

    #[error("missing text primitive at index={0}")]
    MissingTextPrim(u8),
    #[error("expected array")]
    ExpectedArray,

    #[error("expected serialised json: {0}")]
    ExpectedSerialisedJson(CmpUsingDisplay<serde_json::Error>),
    #[error("expected arrays of equal length")]
    ArraysMustHaveSameLength,
}
