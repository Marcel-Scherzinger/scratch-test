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
    /// All **valid** blocks (no unsupported or unknown blocks included)
    pub fn iter_blocks(&self) -> impl Iterator<Item = &Rc<BlockWrapper>> {
        self.valid.values()
    }
    /// Get block by [`Id`]
    pub fn get(&self, id: &Id) -> Option<&Rc<BlockWrapper>> {
        self.valid.get(id)
    }
    /// Iterator of all invalid blocks
    ///
    /// Those are:
    ///
    /// - Blocks that are known to be unsupported ([`UnsupportedBlockKind`])
    ///   by this application
    /// - Blocks with an unknown
    ///   [opcode name](https://en.scratch-wiki.info/wiki/List_of_Block_Opcodes)
    pub fn iter_invalid(&self) -> impl Iterator<Item = (&Id, &Rc<JsonCtxError<BlockKindError>>)> {
        self.invalid.iter()
    }
    /// Iterator of [`Id`]s and
    /// [opcode names](https://en.scratch-wiki.info/wiki/List_of_Block_Opcodes)
    /// of invalid blocks that are unknown by the application
    pub fn iter_unknown_blocks(&self) -> impl Iterator<Item = (&Id, &Rc<str>)> {
        self.iter_invalid().filter_map(|(id, e)| {
            if let BlockKindError::UnknownBlock(n) = e.error() {
                Some((id, n))
            } else {
                None
            }
        })
    }

    /// Iterator of [`Id`]s and [`UnsupportedBlockKind`] of invalid blocks
    /// that are known to be unsupported by this application
    pub fn iter_unsupported_blocks(&self) -> impl Iterator<Item = (&Id, &UnsupportedBlockKind)> {
        self.iter_invalid().filter_map(|(id, e)| {
            if let BlockKindError::UnsupportedBlock(n) = e.error() {
                Some((id, n))
            } else {
                None
            }
        })
    }
    /// Iterator of [`Id`]s and [opcode names](https://en.scratch-wiki.info/wiki/List_of_Block_Opcodes)
    /// of valid and invalid blocks
    pub fn ids_with_blocks(&self) -> impl Iterator<Item = (Id, Rc<str>)> {
        use crate::blocks::GetOpcodeUnit;
        self.valid
            .iter()
            .map(|(id, bw)| (id.clone(), bw.inner.get_opcode().to_string().into()))
            .chain(self.invalid.iter().filter_map(|(id, e)| {
                if let BlockKindError::UnsupportedBlock(n) = e.error() {
                    Some((id.clone(), n.get_opcode().to_string().into()))
                } else if let BlockKindError::UnknownBlock(n) = e.error() {
                    Some((id.clone(), n.clone()))
                } else {
                    None
                }
            }))
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

/// This wraps a Scratch [block](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks)
///
/// It stores the unique [`Id`] of the block Scratch has assigned to it,
/// the [`Id`] of the `next` block (especially important for statements)
/// and the `parent` block (`parent` is not actively used in the application).
///
/// **parent** is not the defined to be the previous block
/// (next and parent don't form a double-linked-list)
/// but it can be the next-outer-block in the hierarchy.
/// [See here for details](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks)
#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct BlockWrapper {
    /// The unique id of this block, alphanumeric and special characters are used
    id: Id,
    /// The [`BlockKind`] contains specific information for this kind of block
    inner: BlockKind,
    /// An optional id to the next block following this
    next: Option<Id>,
    /// `parent` is not actively used in the application
    ///
    /// **parent** is not the defined to be the previous block
    /// (next and parent don't form a double-linked-list)
    /// but it can be the next-outer-block in the hierarchy.
    /// [See here for details](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks)
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
