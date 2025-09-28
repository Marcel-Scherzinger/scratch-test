use model::Id;

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("code tried to access variable that wasn't defined: {0:?}")]
    AccessUnknownVariable(Id),
    #[error("code tried to access list that wasn't defined: {0:?}")]
    AccessUnknownList(Id),

    #[error("tried to execute unknown block")]
    ReachedUnknownBlock(Id),

    #[error("pop called on empty program stack")]
    PopOnEmptyProgramStack,

    #[error("program executed more than allowed maximum number of statements")]
    AllowedNumberOfExecutedStmtsExceeded,

    #[error("program nests blocks in a weird way: e. g. statement inside of expression")]
    UnexpectedNestingOfBlocks,

    #[error("program reached condition based loop without body and entered it")]
    ConditionLoopWithoutBodyNeverStops,

    #[error("found wrong block type: e. g. expected expression and got comparison")]
    UnexpectedBlockKind(Id),

    #[error("program used unknown math operator: {0}")]
    UnsupportedMathOperator(String),
}
pub type RResult<T> = Result<T, RunError>;
