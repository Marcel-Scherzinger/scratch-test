use std::{collections::HashMap, rc::Rc};

use super::error::TargetBlocksError;
use crate::ext::{FromJsonExt, JsonCtxError};
use crate::{
    Id,
    blocks::{BlockKind, BlockKindError, UnsupportedBlockKind},
};

#[derive(Debug, PartialEq)]
pub struct TargetBlocks {
    valid: HashMap<Id, Rc<BlockWrapper>>,
    invalid: HashMap<Id, Rc<JsonCtxError<BlockKindError>>>,
}

impl TargetBlocks {
    pub fn iter_blocks(&self) -> impl Iterator<Item = &Rc<BlockWrapper>> {
        self.valid.values()
    }
    pub fn get(&self, id: &Id) -> Option<&Rc<BlockWrapper>> {
        self.valid.get(id)
    }
    pub fn iter_invalid(&self) -> impl Iterator<Item = (&Id, &Rc<JsonCtxError<BlockKindError>>)> {
        self.invalid.iter()
    }
    pub fn iter_unknown_blocks(&self) -> impl Iterator<Item = (&Id, &Rc<str>)> {
        self.iter_invalid().filter_map(|(id, e)| {
            if let BlockKindError::UnknownBlock(n) = e.error() {
                Some((id, n))
            } else {
                None
            }
        })
    }
    pub fn iter_unsupported_blocks(&self) -> impl Iterator<Item = (&Id, &UnsupportedBlockKind)> {
        self.iter_invalid().filter_map(|(id, e)| {
            if let BlockKindError::UnsupportedBlock(n) = e.error() {
                Some((id, n))
            } else {
                None
            }
        })
    }
}

impl crate::ext::FromJsonExt<Self, TargetBlocksError> for TargetBlocks {
    fn from_json_without_ctx(value: &serde_json::Value) -> Result<Self, TargetBlocksError> {
        let dict = value.as_object().ok_or(TargetBlocksError::ExpectedObject)?;

        let (valid, invalid): (Vec<_>, Vec<_>) = dict
            .into_iter()
            .map(|(id, obj): (&String, &serde_json::Value)| {
                let id: Id = id.clone().into();
                match BlockWrapper::from_json_with_ctx(id.clone(), obj) {
                    Ok(b) => Ok((id, b.into())),
                    Err(error) => Err((id, Rc::new(error))),
                }
            })
            .partition(|r| r.is_ok());
        let valid = valid.into_iter().flatten().collect();
        let invalid = invalid.into_iter().flat_map(Result::err).collect();

        Ok(Self { valid, invalid })
    }
}

#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct BlockWrapper {
    id: Id,
    inner: BlockKind,
    next: Option<Id>,
    parent: Option<Id>,
}
impl BlockWrapper {
    pub(crate) fn from_json_without_ctx(
        id: Id,
        obj: &serde_json::Value,
    ) -> Result<Self, BlockKindError> {
        let next = obj["next"].as_str().map(Id::from);
        let parent = obj["parent"].as_str().map(Id::from);
        let inner = BlockKind::from_json_without_ctx(obj)?;
        Ok(Self {
            id,
            inner,
            next,
            parent,
        })
    }
    pub(crate) fn from_json_with_ctx(
        id: Id,
        value: &serde_json::Value,
    ) -> Result<Self, crate::ext::JsonCtxError<BlockKindError>> {
        use crate::ext::WithJsonContextExt;
        Self::from_json_without_ctx(id, value).with_json(value)
    }
}
