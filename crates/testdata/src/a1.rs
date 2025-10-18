use std::rc::Rc;

use testreports::{CategoryTests, MessageAdder, TestReport};

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
            run_with(interp, tests, None);
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
            run_with(interp, tests, Some((1, 5)));
        });

        report.add_category("Beginn bei 0", |tests| {
            for last in 1..=20 {
                run_with(interp, tests, Some((0, last)));
            }
        });

        report.add_category("Negativer Beginn", |tests| {
            for last in 1..=20 {
                run_with(interp, tests, Some((-10, last)));
            }
        });

        report.add_category("Start = Ende", |tests| {
            for border in 1..=20 {
                run_with(interp, tests, Some((border, border)));
            }
        });

        report
    }
}

fn run_with(
    interp: &interpreter::InterpreterBuilder,
    tests: &mut CategoryTests,
    first_n_last: Option<(i64, i64)>,
) {
    let inputs = first_n_last
        .map(|(first, last)| vec![first, last])
        .unwrap_or_default();
    let (first, last) = first_n_last.unwrap_or((1, 5));

    tests.add_test_case(
        interp.prepare().with_answers(inputs),
        |case, messages, out| {
            let error = case.program_error();

            let expected = (first..=last).map(|i| 2 * i * i).sum::<i64>().to_string();

            let output: Vec<Rc<str>> = out.all_output_texts().cloned().collect();

            if out.warn_used_counter_loop() {
                messages.notify(WARN_COUNTER_LOOP);
            }

            if error.is_some() {
                case.set_expected_output([expected.clone()]);
                Err(())?;
            }
            if let Some(last) = output.last()
                && error.is_none()
            {
                if last.as_ref() == expected.as_str() {
                    return Ok(());
                }
                let last = last.trim_end_matches(|s: char| s.is_ascii_punctuation());

                if let Some(prefix) = last.strip_suffix(&expected) {
                    let last_symbol = prefix.chars().last().unwrap_or_default();
                    if last_symbol == ' ' {
                        return Ok(());
                    }
                    if !last_symbol.is_ascii_digit() {
                        messages.notify(HINT_NO_EXTRA_SPACE);
                        // some text or punctuation as prefix
                        return Ok(());
                    }
                }
            }
            case.set_expected_output([expected]);
            Err(().into())
        },
    );
}
