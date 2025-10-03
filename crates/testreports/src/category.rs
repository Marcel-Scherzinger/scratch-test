use std::borrow::Cow;

use crate::{Message, Messages, TestCase, TestReport};

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub(crate) kind: Cow<'static, str>,
    pub(crate) successes: Vec<TestCase>,
    pub(crate) failures: Vec<TestCase>,
    pub(crate) messages: Messages<Category>,
    pub(crate) global_messages: Messages<TestReport>,
}
impl Category {
    pub fn failures(&self) -> impl Iterator<Item = &TestCase> {
        self.failures.iter()
    }
    pub fn successes(&self) -> impl Iterator<Item = &TestCase> {
        self.successes.iter()
    }
    pub fn kind(&self) -> &str {
        &self.kind
    }
    pub fn category_messages(&self) -> impl Iterator<Item = &Message<Category>> {
        self.messages.iter()
    }
}
