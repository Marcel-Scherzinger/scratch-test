use std::{borrow::Cow, collections::BTreeSet};

use crate::{Message, MessageKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Messages<Level>(BTreeSet<Message<Level>>);

impl<Level> Messages<Level> {
    pub(crate) fn new() -> Self {
        Self(Default::default())
    }
    pub fn add(&mut self, msg: impl Into<Message<Level>>) -> &mut Self {
        let msg = msg.into();
        self.0.insert(msg.clone());
        self
    }
    pub fn hint(&mut self, msg: impl Into<Cow<'static, str>>) -> &mut Self {
        self.add(Message {
            kind: MessageKind::Hint,
            msg: msg.into(),
            phantom: std::marker::PhantomData,
        })
    }
    pub fn hint_to_fix(&mut self, msg: impl Into<Cow<'static, str>>) -> &mut Self {
        self.add(Message {
            kind: MessageKind::HintToFix,
            msg: msg.into(),
            phantom: std::marker::PhantomData,
        })
    }
    pub fn warn(&mut self, msg: impl Into<Cow<'static, str>>) -> &mut Self {
        self.add(Message {
            kind: MessageKind::Warning,
            msg: msg.into(),
            phantom: std::marker::PhantomData,
        })
    }
    pub fn iter(&self) -> impl Iterator<Item = &Message<Level>> {
        self.0.iter()
    }
}
