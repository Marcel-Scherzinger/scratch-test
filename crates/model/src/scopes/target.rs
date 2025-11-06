use super::error::TargetError;
use crate::ext::{FromJsonExt, WithJsonContextExt};
use crate::{Error, TargetBlocks, TargetLists, TargetProcedures, TargetVariables};

/// A target is a sprite or the background
///
/// This models a [file format's target object](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Targets)
/// while focussing on the relevant parts for this application.
#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct Target {
    is_stage: bool,
    name: String,
    variables: TargetVariables,
    lists: TargetLists,
    // broadcasts: (), // not implemented yet
    // comments: (),   // not implemented yet
    blocks: TargetBlocks,
    procedures: TargetProcedures,
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
        let procedures = TargetProcedures::new(&blocks);

        Ok(Self {
            is_stage,
            name,
            variables,
            lists,
            blocks,
            procedures,
        })
    }
}
