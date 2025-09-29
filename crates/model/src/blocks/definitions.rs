use crate::{
    ArgumentReporterName, DropdownSelection, Expression, RefBlock, Variable, interpret_json::List,
};

#[derive(Debug)]
pub enum NoopStmtBlockKind {
    DataShowvariable,
    DataShowlist,
    DataHidevariable,
    DataHidelist,
    LooksShow,
}

#[derive(Debug)]
pub enum StmtBlockKind {
    DataSetvariableto {
        variable_to_set: Variable,
        value: Expression,
    },
    DataChangevariableby {
        variable: Variable,
        value: Expression,
    },
    LooksSayforsecs {
        message: Expression,
        secs: Expression,
    },
    LooksThink {
        message: Expression,
    },
    LooksThinkforsecs {
        message: Expression,
        secs: Expression,
    },
    LooksSay {
        message: Expression,
    },
    ControlWait {
        duration: Expression,
    },
    ControlIf {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
    },
    ControlForever {
        substack: Option<RefBlock>,
    },
    ControlStop {
        stop_option: DropdownSelection,
    },
    ControlWaitUntil {
        condition: Option<RefBlock>,
    },
    ControlRepeat {
        times: Expression,
        substack: Option<RefBlock>,
    },
    ControlIfElse {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
        substack2: Option<RefBlock>,
    },
    DataDeleteoflist {
        list: List,
        index: Expression,
    },
    DataDeletealloflist {
        list: List,
    },
    DataInsertatlist {
        list: List,
        index: Expression,
        item: Expression,
    },
    DataReplaceitemoflist {
        list: List,
        index: Expression,
        item: Expression,
    },
    ControlRepeatuntil {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
    },

    SensingAskandwait {
        question: Expression,
    },
    DataAddtolist {
        list: List,
        item: Expression,
    },
}
#[derive(Debug)]
pub enum ExprBlockKind {
    ArgumentReporterStringNumber {
        value: ArgumentReporterName,
    },
    ArgumentReporterBoolean {
        value: ArgumentReporterName,
    },
    OperatorLength {
        string: Expression,
    },
    OperatorJoin {
        string1: Expression,
        string2: Expression,
    },
    OperatorAdd {
        num1: Expression,
        num2: Expression,
    },
    OperatorMultiply {
        num1: Expression,
        num2: Expression,
    },
    OperatorMod {
        num1: Expression,
        num2: Expression,
    },
    OperatorSubtract {
        num1: Expression,
        num2: Expression,
    },
    OperatorDivide {
        num1: Expression,
        num2: Expression,
    },
    OperatorMathop {
        operator: DropdownSelection,
        num: Expression,
    },
    OperatorLetterOf {
        letter: Expression,
        string: Expression,
    },
    OperatorRound {
        num: Expression,
    },
    /* OperatorRandom {
        from: Expression,
        to: Expression,
    }, */
    /*SensingCurrent {
        currentmenu: DropdownSelection,
    },*/
    // SensingTimer,
    SensingAnswer,
    DataItemnumoflist {
        list: List,
        item: Expression,
    },
    DataItemoflist {
        list: List,
        index: Expression,
    },
    RDataList {
        list: List,
    },
    RDataVar {
        variable: Variable,
    },

    DataLengthoflist {
        list: List,
    },
    // SensingDayssince2000,
}
#[derive(Debug)]
pub enum CmpBlockKind {
    OperatorAnd {
        operand1: RefBlock,
        operand2: RefBlock,
    },
    OperatorOr {
        operand1: RefBlock,
        operand2: RefBlock,
    },
    OperatorEquals {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorGt {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorLt {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorContains {
        string1: Expression,
        string2: Expression,
    },
    OperatorNot {
        operand: RefBlock,
    },

    DataListcontainsitem {
        list: List,
        item: Expression,
    },
}

#[derive(Debug, derive_more::From)]
pub enum BlockKind {
    EventWhenflagclicked,
    EventWhenkeypressed { key_option: DropdownSelection },
    Cmp(CmpBlockKind),
    Expr(ExprBlockKind),
    Stmt(StmtBlockKind),
    Noop(NoopStmtBlockKind),
}

pub(super) fn parse_kind(
    opcode: &str,
    inputs: &serde_json::Map<String, serde_json::Value>,
    fields: &serde_json::Map<String, serde_json::Value>,
) -> Result<BlockKind, super::ParseKindError> {
    use super::getter;
    Ok(match opcode {
        "event_whenflagclicked" => BlockKind::EventWhenflagclicked,
        "event_whenkeypressed" => BlockKind::EventWhenkeypressed {
            key_option: getter!(fields."KEY_OPTION" as dropdown)?,
        },
        "control_repeat_until" => StmtBlockKind::ControlRepeatuntil {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        }
        .into(),
        "looks_sayforsecs" => StmtBlockKind::LooksSayforsecs {
            message: getter!(inputs."MESSAGE" as expression)?,
            secs: getter!(inputs."SECS" as expression)?,
        }
        .into(),
        "looks_thinkforsecs" => StmtBlockKind::LooksThinkforsecs {
            message: getter!(inputs."MESSAGE" as expression)?,
            secs: getter!(inputs."SECS" as expression)?,
        }
        .into(),
        "looks_think" => StmtBlockKind::LooksThink {
            message: getter!(inputs."MESSAGE" as expression)?,
        }
        .into(),
        "looks_say" => StmtBlockKind::LooksSay {
            message: getter!(inputs."MESSAGE" as expression)?,
        }
        .into(),
        "operator_gt" => CmpBlockKind::OperatorGt {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        }
        .into(),
        "operator_lt" => CmpBlockKind::OperatorLt {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        }
        .into(),
        "operator_equals" => CmpBlockKind::OperatorEquals {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        }
        .into(),
        "operator_add" => ExprBlockKind::OperatorAdd {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        }
        .into(),
        "control_wait" => StmtBlockKind::ControlWait {
            duration: getter!(inputs."DURATION" as expression)?,
        }
        .into(),
        "operator_mod" => ExprBlockKind::OperatorMod {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        }
        .into(),
        "operator_divide" => ExprBlockKind::OperatorDivide {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        }
        .into(),
        "operator_subtract" => ExprBlockKind::OperatorSubtract {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        }
        .into(),
        "operator_multiply" => ExprBlockKind::OperatorMultiply {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        }
        .into(),
        "operator_and" => CmpBlockKind::OperatorAnd {
            operand1: getter!(inputs."OPERAND1" as blockref)?,
            operand2: getter!(inputs."OPERAND2" as blockref)?,
        }
        .into(),
        "operator_or" => CmpBlockKind::OperatorOr {
            operand1: getter!(inputs."OPERAND1" as blockref)?,
            operand2: getter!(inputs."OPERAND2" as blockref)?,
        }
        .into(),
        "operator_not" => CmpBlockKind::OperatorNot {
            operand: getter!(inputs."OPERAND" as blockref)?,
        }
        .into(),
        "operator_round" => ExprBlockKind::OperatorRound {
            num: getter!(inputs."NUM" as expression)?,
        }
        .into(),
        "operator_length" => ExprBlockKind::OperatorLength {
            string: getter!(inputs."STRING" as expression)?,
        }
        .into(),
        "data_setvariableto" => StmtBlockKind::DataSetvariableto {
            variable_to_set: getter!(fields."VARIABLE" as variableref)?,
            value: getter!(inputs."VALUE" as expression)?,
        }
        .into(),
        "data_changevariableby" => StmtBlockKind::DataChangevariableby {
            variable: getter!(fields."VARIABLE" as variableref)?,
            value: getter!(inputs."VALUE" as expression)?,
        }
        .into(),

        "operator_contains" => CmpBlockKind::OperatorContains {
            string1: getter!(inputs."STRING1" as expression)?,
            string2: getter!(inputs."STRING2" as expression)?,
        }
        .into(),
        "operator_join" => ExprBlockKind::OperatorJoin {
            string1: getter!(inputs."STRING1" as expression)?,
            string2: getter!(inputs."STRING2" as expression)?,
        }
        .into(),
        "operator_letter_of" => ExprBlockKind::OperatorLetterOf {
            letter: getter!(inputs."LETTER" as expression)?,
            string: getter!(inputs."STRING" as expression)?,
        }
        .into(),
        "operator_mathop" => ExprBlockKind::OperatorMathop {
            operator: getter!(fields."OPERATOR" as dropdown)?,
            num: getter!(inputs."NUM" as expression)?,
        }
        .into(),
        /*"sensing_current" => ExprBlockKind::SensingCurrent {
            currentmenu: getter!(fields."CURRENTMENU" as dropdown)?,
        }
        .into(),*/
        // "sensing_dayssince2000" => ExprBlockKind::SensingDayssince2000.into(),
        "sensing_askandwait" => StmtBlockKind::SensingAskandwait {
            question: getter!(inputs."QUESTION" as expression)?,
        }
        .into(),
        "control_if" => StmtBlockKind::ControlIf {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        }
        .into(),
        "control_forever" => StmtBlockKind::ControlForever {
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        }
        .into(),
        "control_repeat" => StmtBlockKind::ControlRepeat {
            times: getter!(inputs."TIMES" as expression)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        }
        .into(),
        "control_stop" => StmtBlockKind::ControlStop {
            stop_option: getter!(fields."STOP_OPTION" as dropdown)?,
        }
        .into(),
        "control_if_else" => StmtBlockKind::ControlIfElse {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
            substack2: getter!(inputs."SUBSTACK2" as optional blockref)?,
        }
        .into(),
        "control_wait_until" => StmtBlockKind::ControlWaitUntil {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
        }
        .into(),
        "data_deleteoflist" => StmtBlockKind::DataDeleteoflist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
        }
        .into(),
        "data_deletealloflist" => StmtBlockKind::DataDeletealloflist {
            list: getter!(fields."LIST" as listref)?,
        }
        .into(),
        "data_lengthoflist" => ExprBlockKind::DataLengthoflist {
            list: getter!(fields."LIST" as listref)?,
        }
        .into(),
        "data_insertatlist" => StmtBlockKind::DataInsertatlist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
            item: getter!(inputs."ITEM" as expression)?,
        }
        .into(),
        "data_replaceitemoflist" => StmtBlockKind::DataReplaceitemoflist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
            item: getter!(inputs."ITEM" as expression)?,
        }
        .into(),
        "data_showvariable" => NoopStmtBlockKind::DataShowvariable.into(),
        "data_showlist" => NoopStmtBlockKind::DataShowlist.into(),
        "data_hidevariable" => NoopStmtBlockKind::DataHidevariable.into(),
        "data_hidelist" => NoopStmtBlockKind::DataHidelist.into(),
        "looks_show" => NoopStmtBlockKind::LooksShow.into(),
        "data_listcontainsitem" => CmpBlockKind::DataListcontainsitem {
            list: getter!(fields."LIST" as listref)?,
            item: getter!(inputs."ITEM" as expression)?,
        }
        .into(),
        "data_itemoflist" => ExprBlockKind::DataItemoflist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
        }
        .into(),
        "data_addtolist" => StmtBlockKind::DataAddtolist {
            list: getter!(fields."LIST" as listref)?,
            item: getter!(inputs."ITEM" as expression)?,
        }
        .into(),
        "data_itemnumoflist" => ExprBlockKind::DataItemnumoflist {
            list: getter!(fields."LIST" as listref)?,
            item: getter!(inputs."ITEM" as expression)?,
        }
        .into(),
        "argument_reporter_string_number" => ExprBlockKind::ArgumentReporterStringNumber {
            value: getter!(fields."VALUE" as argumentreporter)?,
        }
        .into(),
        // "sensing_timer" => ExprBlockKind::SensingTimer.into(),
        "sensing_answer" => ExprBlockKind::SensingAnswer.into(),
        "operator_random"
        | "seinsing_touchingcolor"
        | "sensing_coloristouchingcolor"
        | "control_delete_this_clone"
        | "control_create_clone_of"
        | "control_start_as_clone"
        | "sound_sounds_menu"
        | "sensing_dayssince2000"
        | "sensing_current"
        | "sensing_timer" => {
            return Err(super::ParseKindError::OpcodeUnsupported(opcode.into()));
        }
        // typically this also means unsupported
        other => return Err(super::ParseKindError::OpcodeUnknown(other.to_string())),
    })
}
