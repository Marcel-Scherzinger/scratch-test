use crate::{
    ArgumentReporterName, DropdownSelection, Expression, RefBlock, Variable, interpret_json::List,
};

#[derive(Debug)]
pub enum BlockKind {
    EventWhenflagclicked,
    // SensingDayssince2000,
    DataSetvariableto {
        variable_to_set: Variable,
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
    OperatorAnd {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorOr {
        operand1: Expression,
        operand2: Expression,
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
        operand: Expression,
    },
    ControlRepeatuntil {
        condition: Option<RefBlock>,
        substack: Option<RefBlock>,
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
    SensingCurrent {
        currentmenu: DropdownSelection,
    },
    SensingAskandwait {
        question: Expression,
    },
    SensingTimer,
    SensingAnswer,
    ControlWait {
        duration: Expression,
    },
    ControlIf {
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
    DataListcontainsitem {
        list: List,
        item: Expression,
    },
    DataLengthoflist {
        list: List,
    },
    DataAddtolist {
        list: List,
        item: Expression,
    },
    DataItemnumoflist {
        list: List,
        item: Expression,
    },
    RDataList {
        list: List,
    },
    RDataVar {
        variable: Variable,
    },
    ArgumentReporterStringNumber {
        value: ArgumentReporterName,
    },
    ArgumentReporterBoolean {
        value: ArgumentReporterName,
    },
}

pub(super) fn parse_kind(
    opcode: &str,
    inputs: &serde_json::Map<String, serde_json::Value>,
    fields: &serde_json::Map<String, serde_json::Value>,
) -> Result<BlockKind, super::ParseKindError> {
    use super::getter;
    Ok(match opcode {
        "event_whenflagclicked" => BlockKind::EventWhenflagclicked,
        "control_repeat_until" => BlockKind::ControlRepeatuntil {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        },
        "looks_sayforsecs" => BlockKind::LooksSayforsecs {
            message: getter!(inputs."MESSAGE" as expression)?,
            secs: getter!(inputs."SECS" as expression)?,
        },
        "looks_thinkforsecs" => BlockKind::LooksThinkforsecs {
            message: getter!(inputs."MESSAGE" as expression)?,
            secs: getter!(inputs."SECS" as expression)?,
        },
        "looks_think" => BlockKind::LooksThink {
            message: getter!(inputs."MESSAGE" as expression)?,
        },
        "looks_say" => BlockKind::LooksSay {
            message: getter!(inputs."MESSAGE" as expression)?,
        },
        "operator_gt" => BlockKind::OperatorGt {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        },
        "operator_lt" => BlockKind::OperatorLt {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        },
        "operator_equals" => BlockKind::OperatorEquals {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        },
        "operator_add" => BlockKind::OperatorAdd {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        },
        "control_wait" => BlockKind::ControlWait {
            duration: getter!(inputs."DURATION" as expression)?,
        },
        "operator_mod" => BlockKind::OperatorMod {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        },
        "operator_divide" => BlockKind::OperatorDivide {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        },
        "operator_subtract" => BlockKind::OperatorSubtract {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        },
        "operator_multiply" => BlockKind::OperatorMultiply {
            num1: getter!(inputs."NUM1" as expression)?,
            num2: getter!(inputs."NUM2" as expression)?,
        },
        "operator_and" => BlockKind::OperatorAnd {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        },
        "operator_or" => BlockKind::OperatorOr {
            operand1: getter!(inputs."OPERAND1" as expression)?,
            operand2: getter!(inputs."OPERAND2" as expression)?,
        },
        "operator_not" => BlockKind::OperatorNot {
            operand: getter!(inputs."OPERAND" as expression)?,
        },
        "operator_round" => BlockKind::OperatorRound {
            num: getter!(inputs."NUM" as expression)?,
        },
        "operator_length" => BlockKind::OperatorLength {
            string: getter!(inputs."STRING" as expression)?,
        },
        "data_setvariableto" => BlockKind::DataSetvariableto {
            variable_to_set: getter!(fields."VARIABLE" as variableref)?,
            value: getter!(inputs."VALUE" as expression)?,
        },
        "operator_contains" => BlockKind::OperatorContains {
            string1: getter!(inputs."STRING1" as expression)?,
            string2: getter!(inputs."STRING2" as expression)?,
        },
        "operator_join" => BlockKind::OperatorJoin {
            string1: getter!(inputs."STRING1" as expression)?,
            string2: getter!(inputs."STRING2" as expression)?,
        },
        "operator_letter_of" => BlockKind::OperatorLetterOf {
            letter: getter!(inputs."LETTER" as expression)?,
            string: getter!(inputs."STRING" as expression)?,
        },
        "operator_mathop" => BlockKind::OperatorMathop {
            operator: getter!(fields."OPERATOR" as dropdown)?,
            num: getter!(inputs."NUM" as expression)?,
        },
        "sensing_current" => BlockKind::SensingCurrent {
            currentmenu: getter!(fields."CURRENTMENU" as dropdown)?,
        },
        // "sensing_dayssince2000" => BlockKind::SensingDayssince2000,
        "sensing_askandwait" => BlockKind::SensingAskandwait {
            question: getter!(inputs."QUESTION" as expression)?,
        },
        "control_if" => BlockKind::ControlIf {
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        },
        "control_forever" => BlockKind::ControlForever {
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        },
        "control_repeat" => BlockKind::ControlRepeat {
            times: getter!(inputs."TIMES" as expression)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
        },
        "control_stop" => BlockKind::ControlStop {
            stop_option: getter!(fields."STOP_OPTION" as dropdown)?,
        },
        "control_if_else" => BlockKind::ControlIfElse {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
            substack: getter!(inputs."SUBSTACK" as optional blockref)?,
            substack2: getter!(inputs."SUBSTACK2" as optional blockref)?,
        },
        "control_wait_until" => BlockKind::ControlWaitUntil {
            condition: getter!(inputs."CONDITION" as optional blockref)?,
        },
        "data_deleteoflist" => BlockKind::DataDeleteoflist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
        },
        "data_lengthoflist" => BlockKind::DataLengthoflist {
            list: getter!(fields."LIST" as listref)?,
        },
        "data_insertatlist" => BlockKind::DataInsertatlist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
            item: getter!(inputs."ITEM" as expression)?,
        },
        "data_replaceitemoflist" => BlockKind::DataReplaceitemoflist {
            list: getter!(fields."LIST" as listref)?,
            index: getter!(inputs."INDEX" as expression)?,
            item: getter!(inputs."ITEM" as expression)?,
        },
        "data_listcontainsitem" => BlockKind::DataListcontainsitem {
            list: getter!(fields."LIST" as listref)?,
            item: getter!(inputs."ITEM" as expression)?,
        },
        "data_addtolist" => BlockKind::DataAddtolist {
            list: getter!(fields."LIST" as listref)?,
            item: getter!(inputs."ITEM" as expression)?,
        },
        "data_itemnumoflist" => BlockKind::DataItemnumoflist {
            list: getter!(fields."LIST" as listref)?,
            item: getter!(inputs."ITEM" as expression)?,
        },
        "argument_reporter_string_number" => BlockKind::ArgumentReporterStringNumber {
            value: getter!(fields."VALUE" as argumentreporter)?,
        },
        "sensing_timer" => BlockKind::SensingTimer,
        "sensing_answer" => BlockKind::SensingAnswer,
        "operator_random"
        | "seinsing_touchingcolor"
        | "sensing_coloristouchingcolor"
        | "control_delete_this_clone"
        | "control_create_clone_of"
        | "control_start_as_clone"
        | "sound_sounds_menu"
        | "sensing_dayssince2000" => {
            return Err(super::ParseKindError::OpcodeUnsupported(opcode.into()));
        }
        // typically this also means unsupported
        other => return Err(super::ParseKindError::OpcodeUnknown(other.to_string())),
    })
}
