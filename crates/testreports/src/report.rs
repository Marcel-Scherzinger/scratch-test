use crate::{Category, CategoryTests, Message, TestCase};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestReport {
    pub(crate) categories: Vec<Category>,
}

impl TestReport {
    pub fn new() -> Self {
        Self { categories: vec![] }
    }
    pub fn add_category(
        &mut self,
        kind: &'static str,
        run: impl FnOnce(&mut CategoryTests),
    ) -> &mut Self {
        let mut category_tests = CategoryTests::new();
        run(&mut category_tests);
        let (successes, failures, messages) = category_tests.take_compressed();
        self.categories.push(Category {
            kind: kind.into(),
            successes,
            failures,
            messages,
        });
        self
    }
    pub fn overall_successes(&self) -> impl Iterator<Item = &TestCase> {
        self.categories.iter().flat_map(|c| c.successes.iter())
    }
    pub fn overall_failures(&self) -> impl Iterator<Item = &TestCase> {
        self.categories.iter().flat_map(|c| c.failures.iter())
    }
    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.categories.iter()
    }
    pub fn global_messages(&self) -> impl Iterator<Item = &Message<TestReport>> {
        use itertools::Itertools;
        self.categories
            .iter()
            .flat_map(|c| c.messages.report().iter())
            .unique()
    }
}
