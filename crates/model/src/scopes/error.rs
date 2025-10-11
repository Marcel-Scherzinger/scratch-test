use crate::Id;

#[derive(Debug, thiserror::Error)]
pub enum TargetError {
    #[error("The attribute \"isStage\" of the target is missing")]
    MissingIsStage,
    #[error("The attribute \"name\" of the target is missing")]
    MissingName,
    #[error("The document doesn't contain a \"targets\" array")]
    NoTargetsArray,
}

#[derive(Debug, thiserror::Error)]
pub enum TargetBlocksError {
    #[error("expected object {{...}} for blocks of target")]
    ExpectedObject,
    /* #[error("at least one target block (id={id:?}) has unknown structure (block-error={error})")]
    AtLeastOneInvalid {
        id: Id,
        error: JsonCtxError<BlockKindError>,
    }, */
}

#[derive(Debug, thiserror::Error)]
pub enum TargetListsError {
    #[error("expected object {{...}} for lists of target")]
    ExpectedObject,
    #[error("at least one target list (id={0:?}) has unknown structure")]
    AtLeastOneInvalid(Id),
}

#[derive(Debug, thiserror::Error)]
pub enum TargetVariablesError {
    #[error("expected object {{...}} for variables of target")]
    ExpectedObject,
    #[error("at least one target variable (id={0:?}) has unknown structure")]
    AtLeastOneInvalid(Id),
}
