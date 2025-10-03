use std::rc::Rc;

use testreports::{CategoryTests, TestCase, TestReport};

use crate::defs::*;

/// output sum 2 * i * i of integers i from 1 to 5
///
/// - no inputs
/// - only last output used
///     - debug outputs (except last) ignored
pub struct A1a;

/// output sum 2 * i * i of integers i from first to last
///
/// - two inputs (sorted in size)
/// - only last output used
///     - debug outputs (except last) ignored
pub struct A1b;

impl ExerciseTest for A1a {
    fn exercise(&self) -> (u8, ExercisePart) {
        (1, ExercisePart::A)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut report = TestReport::new();
        report.add_category("", |tests| {
            tests.add_result_of(|tests| run_with(interp, tests, Some((1, 5))));
        });
        report
    }
}

impl ExerciseTest for A1b {
    fn exercise(&self) -> (u8, ExercisePart) {
        (1, ExercisePart::B)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut report = TestReport::new();

        report.add_category("Beispiel", |tests| {
            tests.add_result_of(|tests| run_with(interp, tests, Some((1, 5))));
        });

        report.add_category("Beginn bei 0", |tests| {
            for last in 1..=20 {
                tests.add_result_of(|tests| run_with(interp, tests, Some((0, last))));
            }
        });

        report.add_category("Negativer Beginn", |tests| {
            for last in 1..=20 {
                tests.add_result_of(|tests| run_with(interp, tests, Some((-10, last))));
            }
        });

        report.add_category("Start = Ende", |tests| {
            for border in 1..=20 {
                tests.add_result_of(|tests| run_with(interp, tests, Some((border, border))));
            }
        });

        report
    }
}

fn run_with(
    interp: &interpreter::InterpreterBuilder,
    tests: &mut CategoryTests,
    first_n_last: Option<(i64, i64)>,
) -> Result<TestCase, TestCase> {
    let inputs = first_n_last
        .map(|(first, last)| vec![first, last])
        .unwrap_or_default();
    let (first, last) = first_n_last.unwrap_or((1, 5));

    let mut test_case = tests.start(interp.prepare().with_answers(inputs));
    let error = test_case.program_error();

    let expected = (first..=last).map(|i| 2 * i * i).sum::<i64>().to_string();

    let output: Vec<Rc<str>> = test_case.out().all_output_texts().cloned().collect();

    if test_case.out().warn_used_counter_loop() {
        tests.global_message(WARN_COUNTER_LOOP);
    }

    if error.is_some() {
        test_case.set_expected_output([expected]);
        return Err(test_case);
    }
    if let Some(last) = output.last()
        && error.is_none()
    {
        if last.as_ref() == expected.as_str() {
            return Ok(test_case);
        }
        let last = last.trim_end_matches(|s: char| s.is_ascii_punctuation());

        if let Some(prefix) = last.strip_suffix(&expected) {
            let last_symbol = prefix.chars().last().unwrap_or_default();
            if last_symbol == ' ' {
                return Ok(test_case);
            }
            if !last_symbol.is_ascii_digit() {
                tests.global_message(HINT_NO_EXTRA_SPACE);
                // some text or punctuation as prefix
                return Ok(test_case);
            }
        }
    }
    test_case.set_expected_output([expected]);
    Err(test_case)
}
