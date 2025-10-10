use std::rc::Rc;

use crate::{
    BlockKindError, Error, Id, TargetBlocks, TargetLists, TargetVariables, UnsupportedBlockKind,
    ext::FromJsonExt,
    ext::{JsonCtxError, WithJsonContextExt},
};

/// A target is a sprite or the background
#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct Target {
    is_stage: bool,
    name: String,
    variables: TargetVariables,
    lists: TargetLists,
    // broadcasts: (), // not implemented yet
    // comments: (),   // not implemented yet
    blocks: TargetBlocks,
}

#[derive(Debug, thiserror::Error)]
pub enum TargetError {
    #[error("The attribute \"isStage\" of the target is missing")]
    MissingIsStage,
    #[error("The attribute \"name\" of the target is missing")]
    MissingName,
    #[error("The document doesn't contain a \"targets\" array")]
    NoTargetsArray,
}

impl Target {
    pub(crate) fn from_json(value: &serde_json::Value) -> Result<Self, Error> {
        let is_stage = value["isStage"]
            .as_bool()
            .ok_or(TargetError::MissingIsStage)
            .with_json(value)?;
        let name = value["name"]
            .as_str()
            .ok_or(TargetError::MissingName)
            .with_json(value)?
            .into();
        let variables = TargetVariables::from_json_with_ctx(&value["variables"])?;
        let lists = TargetLists::from_json_with_ctx(&value["lists"])?;
        let blocks = TargetBlocks::from_json_without_ctx(&value["blocks"])?;
        Ok(Self {
            is_stage,
            name,
            variables,
            lists,
            blocks,
        })
    }
}

#[derive(Debug, derive_getters::Getters, Clone, PartialEq)]
pub struct ProjectDoc {
    pub(crate) targets: Rc<[Target]>,
    pub(crate) semver: Option<Rc<str>>,
}
impl ProjectDoc {
    pub fn invalid_blocks(&self) -> impl Iterator<Item = (&Id, &Rc<JsonCtxError<BlockKindError>>)> {
        self.targets.iter().flat_map(|t| t.blocks().iter_invalid())
    }
    pub fn unsupported_blocks(&self) -> impl Iterator<Item = (&Id, &UnsupportedBlockKind)> {
        self.targets
            .iter()
            .flat_map(|t| t.blocks().iter_unsupported_blocks())
    }
    pub fn unknown_blocks(&self) -> impl Iterator<Item = (&Id, &Rc<str>)> {
        self.targets
            .iter()
            .flat_map(|t| t.blocks().iter_unknown_blocks())
    }
    pub fn ensure_no_invalid_blocks(self) -> Result<Self, Self> {
        if self.invalid_blocks().next().is_some() {
            Err(self)
        } else {
            Ok(self)
        }
    }
    pub fn ensure_no_unknown_blocks(self) -> Result<Self, Self> {
        if self.unknown_blocks().next().is_some() {
            Err(self)
        } else {
            Ok(self)
        }
    }
    pub fn ensure_no_unsupported_blocks(self) -> Result<Self, Self> {
        if self.unsupported_blocks().next().is_some() {
            Err(self)
        } else {
            Ok(self)
        }
    }
}
