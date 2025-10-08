mod adder;
mod hub;
mod messages;
mod traits;

use std::{borrow::Cow, hash::Hash};

pub use adder::MessageAdder;
pub use hub::MessageHub;
pub use messages::Messages;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, derive_more::Display)]
pub enum MessageKind {
    #[display("hint")]
    Hint,
    #[display("hint-to-fix")]
    HintToFix,
    #[display("warning")]
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
    pub const fn chint(msg: &'static str) -> Self {
        Self {
            kind: MessageKind::Hint,
            msg: Cow::Borrowed(msg),
            phantom: std::marker::PhantomData,
        }
    }
    pub const fn chint_to_fix(msg: &'static str) -> Self {
        Self {
            kind: MessageKind::HintToFix,
            msg: Cow::Borrowed(msg),
            phantom: std::marker::PhantomData,
        }
    }
    pub const fn cwarning(msg: &'static str) -> Self {
        Self {
            kind: MessageKind::Warning,
            msg: Cow::Borrowed(msg),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn warning(msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind: MessageKind::Warning,
            msg: msg.into(),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn hint(msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind: MessageKind::Hint,
            msg: msg.into(),
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
