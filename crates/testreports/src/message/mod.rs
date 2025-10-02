mod messages;
mod traits;

use std::{borrow::Cow, hash::Hash};

pub use messages::Messages;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum MessageKind {
    Hint,
    HintToFix,
    Warning,
}

pub struct Message<Level> {
    kind: MessageKind,
    msg: Cow<'static, str>,
    phantom: std::marker::PhantomData<Level>,
}

impl<Level> std::fmt::Debug for Message<Level> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Message {{ kind: {:?}, msg: {:?} }}",
            self.kind, self.msg
        ))
    }
}

impl<Level> Message<Level> {
    pub const fn hint(msg: &'static str) -> Self {
        Self {
            kind: MessageKind::Hint,
            msg: Cow::Borrowed(msg),
            phantom: std::marker::PhantomData,
        }
    }
    pub const fn hint_to_fix(msg: &'static str) -> Self {
        Self {
            kind: MessageKind::HintToFix,
            msg: Cow::Borrowed(msg),
            phantom: std::marker::PhantomData,
        }
    }
    pub const fn warning(msg: &'static str) -> Self {
        Self {
            kind: MessageKind::Warning,
            msg: Cow::Borrowed(msg),
            phantom: std::marker::PhantomData,
        }
    }
    pub const fn kind(&self) -> &MessageKind {
        &self.kind
    }
    pub fn msg(&self) -> &str {
        &self.msg
    }
}
