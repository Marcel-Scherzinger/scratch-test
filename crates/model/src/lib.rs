/// copied from [https://github.com/scratchfoundation/scratch-vm/blob/develop/src/serialization/sb3.js]
pub mod constants;

mod blocks;
mod error;
mod ext;
mod interpret_json;
mod reader;
mod scopes;
mod scratch_expr;

use std::rc::Rc;

pub use blocks::{
    BlockAttrError, BlockKind, BlockKindError, CmpBlockKind, CmpBlockKindUnit, EventBlockKind,
    ExprBlockKind, StmtBlockKind,
};
pub use error::{DocError, Error};
use ext::*;
use interpret_json::*;
pub use interpret_json::{Expression, List, Variable};
pub use scopes::*;
pub use scratch_expr::{IntegerOutOfBounds, ScratchExpr};

pub use scratch_expr::SValue as VariableValue;
pub use scratch_expr::SValue;

pub type Id = Rc<str>;
pub type OpcodeNum = u64;

#[derive(derive_more::Debug, PartialEq, derive_more::Deref, derive_more::From)]
#[debug("{_0:?}")]
pub struct RefBlock(Id);

#[derive(derive_more::Debug, PartialEq, derive_more::Deref, derive_more::From)]
#[debug("{_0:?}")]
pub struct DropdownSelection(Rc<str>);

#[derive(derive_more::Debug, PartialEq, derive_more::Deref, derive_more::From)]
#[debug("{_0:?}")]
pub struct ArgumentReporterName(Rc<str>);

impl RefBlock {
    pub fn id(&self) -> &Id {
        &self.0
    }
    pub fn o_id(&self) -> Id {
        self.0.clone()
    }
}

macro_rules! impl_string_from {
    ($type: ty, $inter: ty) => {
        impl<'a> From<&'a str> for $type {
            fn from(val: &'a str) -> Self {
                let r: $inter = val.into();
                r.into()
            }
        }
    };
}
impl_string_from!(RefBlock, Rc<str>);
impl_string_from!(DropdownSelection, Rc<str>);
impl_string_from!(ArgumentReporterName, Rc<str>);

impl ProjectDoc {
    pub fn from_json(doc: serde_json::Value) -> Result<ProjectDoc, Error> {
        let semver = doc["meta"]["semver"].as_str().map(Rc::from);
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
