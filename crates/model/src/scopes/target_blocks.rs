use std::collections::HashMap;

use crate::{BlockKind, BlockKindError, FromJsonExt, Id, ext::JsonCtxError};

#[derive(Debug, thiserror::Error)]
pub enum TargetBlocksError {
    #[error("expected object {{...}} for blocks of target")]
    ExpectedObject,
    #[error("at least one target block (id={id:?}) has unknown structure (block-error={error})")]
    AtLeastOneInvalid {
        id: Id,
        error: JsonCtxError<BlockKindError>,
    },
}

#[derive(Debug)]
pub struct TargetBlocks {
    map: HashMap<Id, BlockWrapper>,
}
impl crate::FromJsonExt<Self, TargetBlocksError> for TargetBlocks {
    fn from_json_without_ctx(value: &serde_json::Value) -> Result<Self, TargetBlocksError> {
        let dict = value.as_object().ok_or(TargetBlocksError::ExpectedObject)?;

        let map: Result<_, TargetBlocksError> = dict
            .into_iter()
            .map(|(id, obj): (&String, &serde_json::Value)| {
                let id: Id = id.clone().into();
                match BlockWrapper::from_json_with_ctx(id.clone(), obj) {
                    Ok(b) => Ok((id, b)),
                    Err(error) => Err(TargetBlocksError::AtLeastOneInvalid { id, error }),
                }
            })
            .collect();

        Ok(Self { map: map? })
    }
}

#[derive(Debug)]
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
    ) -> Result<Self, crate::JsonCtxError<BlockKindError>> {
        use crate::WithJsonContextExt;
        Self::from_json_without_ctx(id, value).with_json(value)
    }
}
