use crate::{
    Error, FromJsonExt, TargetBlocks, TargetLists, TargetVariables, ext::WithJsonContextExt,
};

/// A target is a sprite or the background
#[derive(Debug, derive_getters::Getters)]
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

#[derive(Debug, derive_getters::Getters)]
pub struct ProjectDoc {
    pub(crate) targets: Vec<Target>,
    pub(crate) semver: Option<String>,
}
