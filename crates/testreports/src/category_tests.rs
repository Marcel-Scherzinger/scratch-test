use std::{collections::HashMap, rc::Rc};

use crate::{Category, Message, MessageAdder, MessageHub, TestCase, TestReport};

pub struct CategoryTests {
    messages: MessageHub<true, true, false>,
    successes: Vec<TestCase>,
    failures: Vec<TestCase>,
}

impl<L> MessageAdder<L> for CategoryTests
where
    MessageHub<true, true, false>: MessageAdder<L>,
{
    fn notify(&mut self, message: Message<L>) {
        self.messages.notify(message);
    }
}

#[derive(Debug, derive_more::From)]
pub enum TestCaseFailureDetails {
    Unit(()),
    CaseMessage(Message<TestCase>),
    CatMessage(Message<Category>),
    ReportMessage(Message<TestReport>),
}

impl CategoryTests {
    pub(crate) fn new() -> Self {
        Self {
            messages: MessageHub::new(),
            successes: vec![],
            failures: vec![],
        }
    }
    pub(crate) fn take_compressed(
        mut self,
    ) -> (Vec<TestCase>, Vec<TestCase>, MessageHub<true, true, false>) {
        for succ in self.successes.iter_mut().chain(self.failures.iter_mut()) {
            self.messages
                .report_mut()
                .take_and_remove_from(&mut succ.messages.report);
            self.messages
                .category_mut()
                .take_and_remove_from(&mut succ.messages.category);
        }

        (self.successes, self.failures, self.messages)
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

    pub fn add_test_case(
        &mut self,
        prepared: interpreter::PrepareInterpreter,
        run: impl FnOnce(
            &mut TestCase,
            &mut MessageHub,
            Rc<interpreter::InterpreterReport>,
        ) -> Result<(), TestCaseFailureDetails>,
    ) -> &mut Self {
        let mut test_case = self.start(prepared);
        let out = test_case.out().clone();

        let result = {
            let mut message_hub = MessageHub::new();

            let res = run(&mut test_case, &mut message_hub, out);
            test_case
                .messages
                .case
                .take_and_remove_from(&mut message_hub.case);
            test_case
                .messages
                .category
                .take_and_remove_from(&mut message_hub.category);
            test_case
                .messages
                .report
                .take_and_remove_from(&mut message_hub.report);
            res
        };

        match result {
            Ok(()) => self.add_success(test_case),
            Err(det) => {
                use TestCaseFailureDetails as T;
                match det {
                    T::Unit(_) => (),
                    T::CatMessage(msg) => test_case.notify(msg),
                    T::CaseMessage(msg) => test_case.notify(msg),
                    T::ReportMessage(msg) => test_case.notify(msg),
                }

                self.add_failure(test_case)
            }
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
        self.messages.notify(msg);
    }
    pub fn global_message(&mut self, msg: Message<TestReport>) {
        self.messages.report_mut().add(msg);
    }

    pub fn start(&mut self, inter: interpreter::PrepareInterpreter) -> TestCase {
        let finished = inter.start();

        TestCase {
            data_lists: HashMap::new(),
            expected_output: None,
            interpreter: std::rc::Rc::new(finished),
            messages: MessageHub::new(),
        }
    }
}
