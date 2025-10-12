use std::{borrow::Cow, rc::Rc};

use crate::{blocks::definitions::UnsupportedBlockKind, interpret_json::FormatError};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BlockKindError {
    #[error("format-error: {0}")]
    Format(#[from] Box<FormatError>),
    /// The json object is not parsable as a block
    #[error("the provided json object is no valid block")]
    InvalidBlockType,
    /// Every block should contain an opcode.
    /// Some blocks like variable reading are stored as arrays with numeric opcodes
    #[error("the kind of this array block (opcode) is missing")]
    NoOpcodeInArr,
    /// Every block should contain an opcode
    #[error("the kind of this block (opcode) is missing")]
    NoOpcode,
    /// Every block should contain an entry "inputs" pointing to a json object
    #[error("the \"inputs\" object field of this block (kind={block_kind}) is missing")]
    NoInputs { block_kind: String },
    /// Every block should contain an entry "fields" pointing to a json object
    #[error("the \"fields\" object field of this block (kind={block_kind}) is missing")]
    NoFields { block_kind: String },
    /// Every procedures block should contain an entry "mutation" pointing to a json object
    #[error("the \"mutation\" object field of this block (kind={block_kind}) is missing")]
    NoMutation { block_kind: String },

    /// The block is unknown
    #[error("the provided block ({0:?}) is unknown")]
    UnknownBlock(Rc<str>),
    /// The block is known and (intentionally) unsupported.
    /// Programs containing this block get rejected.
    #[error("the provided block with opcode ({0:?}) is unsupperted")]
    UnsupportedBlock(UnsupportedBlockKind),
    /// The block is missing an attribute that is needed for the block's type.
    #[error("block (kind={block_kind}) {error}")]
    Missing {
        block_kind: String,
        error: Box<BlockAttrError>,
    },
}

impl From<FormatError> for BlockKindError {
    fn from(value: FormatError) -> Self {
        BlockKindError::Format(value.into())
    }
}

#[derive(Debug, derive_more::Display, PartialEq)]
pub enum BlockAttrError {
    #[display(
        "doesn't contain required attribute {attr_name:?} in {source:?} when treated as {treated_as:?}: {error}"
    )]
    Invalid {
        /// The attribute name
        attr_name: Cow<'static, str>,
        /// The way the attribute was interpreted: as expression, block reference, ...
        treated_as: &'static str,
        /// If it was read from "inputs", "fields" or mutation
        source: &'static str,
        /// The exact error that occured
        error: crate::error::FormatError,
    },
    #[display(
        "doesn't contain required attribute {attr_name:?} in {source:?} when treated as {treated_as:?}"
    )]
    Missing {
        /// The attribute name
        attr_name: Cow<'static, str>,
        /// The way the attribute was interpreted: as expression, block reference, ...
        treated_as: &'static str,
        /// If it was read from "inputs", "fields" or "mutation"
        source: &'static str,
    },
}
impl std::error::Error for BlockAttrError {}

#[derive(derive_more::From)]
pub(super) enum ParseKindError {
    MissingAttr(super::BlockAttrError),
    #[from(skip)]
    OpcodeUnknown(Rc<str>),
    OpcodeUnsupported(UnsupportedBlockKind),
    BlockKind(BlockKindError),
}
