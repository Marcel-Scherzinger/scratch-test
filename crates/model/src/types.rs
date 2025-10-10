use std::rc::Rc;

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
