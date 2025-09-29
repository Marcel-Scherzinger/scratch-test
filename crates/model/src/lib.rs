#![allow(unused)]

/// copied from [https://github.com/scratchfoundation/scratch-vm/blob/develop/src/serialization/sb3.js]
pub mod constants;

mod blocks;
mod error;
mod ext;
mod interpret_json;
mod reader;
mod scopes;
mod scratch_expr;

pub use blocks::{
    BlockAttrError, BlockKind, BlockKindError, CmpBlockKind, ExprBlockKind, StmtBlockKind,
};
pub use error::{DocError, Error};
use ext::*;
use interpret_json::*;
pub use interpret_json::{Expression, List, Variable};
pub use scopes::*;
pub use scratch_expr::{IntegerOutOfBounds, ScratchExpr};

pub use scratch_expr::SValue as VariableValue;

pub type Id = std::rc::Rc<str>;
pub type OpcodeNum = u64;
pub type RefBlock = Id;
pub type DropdownSelection = String;
pub type ArgumentReporterName = String;

impl ProjectDoc {
    pub fn from_json(doc: serde_json::Value) -> Result<ProjectDoc, Error> {
        let semver = doc["meta"]["semver"].as_str().map(str::to_string);
        let targets = doc["targets"]
            .as_array()
            .ok_or(TargetError::NoTargetsArray)
            .with_json(&doc)?;
        let targets: Result<Vec<Target>, _> = targets.iter().map(Target::from_json).collect();
        Ok(ProjectDoc {
            targets: targets?.into(),
            semver,
        })
    }
}
