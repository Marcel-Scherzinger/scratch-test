use crate::{Category, MessageAdder, Messages, TestCase, TestReport};

#[derive(Debug, Clone, PartialEq)]
pub struct MessageHub<const REP: bool = true, const CAT: bool = true, const CASE: bool = true> {
    pub(crate) report: Messages<TestReport>,
    pub(crate) category: Messages<Category>,
    pub(crate) case: Messages<TestCase>,
}

impl<const A: bool, const B: bool, const C: bool> MessageHub<A, B, C> {
    pub(crate) fn new() -> Self {
        Self {
            report: Messages::new(),
            category: Messages::new(),
            case: Messages::new(),
        }
    }
}

impl<const A: bool, const B: bool> MessageHub<true, A, B> {
    pub(crate) fn report_mut(&mut self) -> &mut Messages<TestReport> {
        &mut self.report
    }
    pub(crate) fn report(&self) -> &Messages<TestReport> {
        &self.report
    }
}
impl<const A: bool, const B: bool> MessageHub<A, true, B> {
    pub(crate) fn category_mut(&mut self) -> &mut Messages<Category> {
        &mut self.category
    }
    pub(crate) fn category(&self) -> &Messages<Category> {
        &self.category
    }
}
#[allow(unused)]
impl<const A: bool, const B: bool> MessageHub<A, B, true> {
    pub(crate) fn case_mut(&mut self) -> &mut Messages<TestCase> {
        &mut self.case
    }
    pub(crate) fn case(&self) -> &Messages<TestCase> {
        &self.case
    }
}

impl<const A: bool, const B: bool, const C: bool> Default for MessageHub<A, B, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const A: bool, const B: bool> MessageAdder<TestCase> for MessageHub<A, B, true> {
    fn notify(&mut self, message: super::Message<TestCase>) {
        self.case.notify(message);
    }
}

impl<const A: bool, const B: bool> MessageAdder<Category> for MessageHub<A, true, B> {
    fn notify(&mut self, message: super::Message<Category>) {
        self.category.notify(message);
    }
}

impl<const A: bool, const B: bool> MessageAdder<TestReport> for MessageHub<true, A, B> {
    fn notify(&mut self, message: super::Message<TestReport>) {
        self.report.notify(message);
    }
}
