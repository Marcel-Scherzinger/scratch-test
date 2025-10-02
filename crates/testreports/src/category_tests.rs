use std::collections::HashMap;

use crate::{Category, Message, Messages, TestCase, TestReport};

pub struct CategoryTests {
    global_messages: Messages<TestReport>,
    messages: Messages<Category>,
    successes: Vec<TestCase>,
    failures: Vec<TestCase>,
}
impl CategoryTests {
    pub(crate) fn new() -> Self {
        Self {
            messages: Messages::new(),
            successes: vec![],
            failures: vec![],
            global_messages: Messages::new(),
        }
    }
    pub(crate) fn take(
        self,
    ) -> (
        Vec<TestCase>,
        Vec<TestCase>,
        Messages<Category>,
        Messages<TestReport>,
    ) {
        (
            self.successes,
            self.failures,
            self.messages,
            self.global_messages,
        )
    }
    pub fn add_result(&mut self, case: Result<TestCase, TestCase>) -> &mut Self {
        match case {
            Ok(case) => self.add_success(case),
            Err(case) => self.add_failure(case),
        }
    }
    pub fn add_result_of(
        &mut self,
        case: impl FnOnce(&mut Self) -> Result<TestCase, TestCase>,
    ) -> &mut Self {
        match case(self) {
            Ok(case) => self.add_success(case),
            Err(case) => self.add_failure(case),
        }
    }
    pub fn add_success(&mut self, case: TestCase) -> &mut Self {
        self.successes.push(case);
        self
    }
    pub fn add_failure(&mut self, case: TestCase) -> &mut Self {
        self.failures.push(case);
        self
    }
    pub fn category_message(&mut self, msg: Message<Category>) {
        self.messages.add(msg);
    }
    pub fn global_message(&mut self, msg: Message<TestReport>) {
        self.global_messages.add(msg);
    }

    pub fn start(&mut self, inter: interpreter::PrepareInterpreter) -> TestCase {
        let finished = inter.start();

        TestCase {
            data_lists: HashMap::new(),
            expected_output: None,
            interpreter: finished,
            messages: Messages::new(),
        }
    }
}
