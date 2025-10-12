use itertools::Itertools;

use super::Target;
use crate::error::JsonCtxError;
use crate::{
    Id,
    blocks::{BlockKindError, UnsupportedBlockKind},
};
use std::rc::Rc;

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

    pub fn ids_with_blocks(&self) -> impl Iterator<Item = (Id, Rc<str>)> {
        self.targets()
            .iter()
            .flat_map(|t| t.blocks().ids_with_blocks())
    }

    pub fn su_ids_with_blocks(&self) -> impl Iterator<Item = (Id, Rc<str>)> {
        self.ids_with_blocks().sorted().unique()
    }
}
