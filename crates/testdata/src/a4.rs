mod inputs;
mod isbn_logic;
#[cfg(test)]
mod tests;

use isbn_logic::{Isbn, check_isbn};

use inputs::{
    CORRECT_LENGTH_WRONG_PATTERN, VALID_ISBN_INPUTS, WRONG_LENGTH_CORRECT_PATTERN,
    WRONG_LENGTH_WRONG_PATTERN,
};

use testreports::{Category, CategoryTests, Message, MessageAdder, TestCase, TestReport};

use crate::defs::*;

const MULTIPLE_OUTPUTS: Message<Category> =
    Message::cwarning("Your program outputs more than one answer. Only the last will be used");

const AMBIGUOUS_ANSWER: Message<Category> = Message::cwarning(
    "Didn't find marker word 'korrekt'/'richtig'/'gültig' (→ valid) or 'nicht'/'falsch'/'inkorrekt' (→ invalid). ('nicht korrect' would be interpreted as invalid)",
);
const EXPECTED_NOT_ONLY_CORRECT: Message<TestReport> = Message::chint(
    "The 'expected answer' is not the only allowed format for output. You can use it (or a string containing the correct marker word)",
);

pub struct A4;

impl ExerciseTest for A4 {
    fn exercise(&self) -> (u8, ExercisePart) {
        (4, ExercisePart::A)
    }
    fn run(&self, interp: &interpreter::InterpreterBuilder) -> TestReport {
        let mut report = TestReport::new();

        report.add_category("valid isbn inputs", move |tests| {
            for valid in VALID_ISBN_INPUTS {
                run_with(interp, tests, valid);
            }
        });
        report.add_category("invalid pattern, length is 13", move |tests| {
            for invalid in CORRECT_LENGTH_WRONG_PATTERN {
                run_with(interp, tests, invalid);
            }
        });
        report.add_category("invalid pattern, length is not 13", move |tests| {
            for invalid in WRONG_LENGTH_WRONG_PATTERN {
                run_with(interp, tests, invalid);
            }
        });
        report.add_category("valid pattern, length is not 13", move |tests| {
            for invalid in WRONG_LENGTH_CORRECT_PATTERN {
                run_with(interp, tests, invalid);
            }
        });

        report
    }
}

fn run_with(interp: &interpreter::InterpreterBuilder, tests: &mut CategoryTests, isbn: Isbn) {
    let validated_isbn = check_isbn(isbn);

    tests.add_test_case(
        interp.prepare().with_answers([isbn]),
        |test_case, messages, out| {
            if out.all_output_texts().count() > 1 {
                messages.notify(MULTIPLE_OUTPUTS);
            }
            let last_output = if let Some(lo) = out.all_output_texts().last() {
                lo
            } else {
                // No output at all.
                set_expected(test_case, isbn, validated_isbn.is_ok());
                return Err(().into());
            };

            // try to guess format from `last_output`

            if last_output.contains("inkorrekt")
                || last_output.contains("nicht")
                || last_output.contains("falsch")
                || last_output.contains("ungültig")
            {
                // program says: invalid
                if validated_isbn.is_err() {
                    return Ok(());
                }
            } else if last_output.contains("korrekt")
                || last_output.contains("richtig")
                || last_output.contains("gültig")
            {
                // program says: valid
                if validated_isbn.is_ok() {
                    return Ok(());
                }
            } else {
                messages.notify(AMBIGUOUS_ANSWER);
            }
            set_expected(test_case, isbn, validated_isbn.is_ok());

            Err(().into())
        },
    );
}

// Expected is hint how to format it.
// Other formats are accepted as well.
fn set_expected(test_case: &mut TestCase, _isbn: Isbn, is_valid: bool) {
    test_case.notify(EXPECTED_NOT_ONLY_CORRECT);
    if is_valid {
        test_case.set_expected_output(["Die ISBN ist korrekt".to_string()]);
    } else {
        test_case.set_expected_output(["Die ISBN ist nicht korrekt".to_string()]);
    }
}
