use model::Id;

#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum RunError {
    #[error("code tried to access variable that wasn't defined: {0:?}")]
    AccessUnknownVariable(Id),
    #[error("code tried to access list that wasn't defined: {0:?}")]
    AccessUnknownList(Id),

    #[error("tried to execute unknown block")]
    ReachedUnknownBlock(Id),

    #[error("pop called on empty program stack")]
    PopOnEmptyProgramStack,

    #[error("pop called on empty argument frames stack")]
    PopOnEmptyArgumentFramesStack,

    #[error("program executed more than allowed maximum number of statements")]
    AllowedNumberOfExecutedStmtsExceeded,

    #[error("program nests blocks in a weird way: e. g. statement inside of expression")]
    UnexpectedNestingOfBlocks,

    #[error("program reached condition based loop without body and entered it")]
    ConditionLoopWithoutBodyNeverStops,

    #[error("program reached infinite loop without body")]
    InfiniteLoopWithoutBodyNeverStops,

    #[error("found wrong block type: e. g. expected expression and got comparison")]
    UnexpectedBlockKind(Id),

    #[error("program used unknown math operator: {0}")]
    UnsupportedMathOperator(String),

    #[error("program called stop block and terminated")]
    TerminateBecauseOfStop,

    #[error("program entered wait until")]
    WaitTillNeverStops,
    #[error("program asked questions but no more answers were predefined")]
    QuestionAskedWithoutAnswer,

    #[error("program tried to call unknown procedure -- could be caused by invalid file")]
    ReachedUnknownProcedure,

    #[error("arguments in procedure call don't match procedure prototype")]
    InvalidProcedureCallArguments,

    #[error("program requested integer random range not matching provided random number")]
    ProvidedRandomOutOfRequestedIntRange { from: i64, to: i64, got: i64 },
    #[error("program requested float random range not matching provided random number")]
    ProvidedRandomOutOfRequestedFloatRange { from: f64, to: f64, got: f64 },
}
pub type RResult<T> = Result<T, RunError>;
