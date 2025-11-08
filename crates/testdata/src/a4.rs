mod inputs;
mod isbn_logic;
#[cfg(test)]
mod tests;

use interpreter::RunError;
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
const ONE_INPUT_CALL_PER_DIGIT: Message<TestReport> = Message::cwarning(
    "Your program asks more than one question so it's assumed that you ask for every digit of the ISBN with a separate input question. (The input-field of test case-outputs reflects the first try with a single input and not the second try with different inputs but the other details reflext the second try.)

This digit input is very inconvenient for users and you should consider chnaging that.",
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

fn isbn2digit_array(mut isbn: Isbn) -> Vec<i64> {
    let mut split = vec![];
    while isbn != 0 {
        split.push(isbn % 10);
        isbn /= 10;
    }
    split.reverse();
    split
}

fn run_with(interp: &interpreter::InterpreterBuilder, tests: &mut CategoryTests, isbn: Isbn) {
    let validated_isbn = check_isbn(isbn);

    let prepared1 = interp.prepare().with_answers([isbn]);

    let mut test_case = tests.start(prepared1);
    let mut out = test_case.out().clone();

    if out.run_error() == Some(&RunError::QuestionAskedWithoutAnswer) {
        // user askes for every digit with a single input question

        let prepared13 = interp.prepare().with_answers(isbn2digit_array(isbn));
        test_case = tests.start(prepared13);
        out = test_case.out().clone();

        test_case.notify(ONE_INPUT_CALL_PER_DIGIT);
    }

    let mut result_call = || {
        if out.all_output_texts().count() > 1 {
            test_case.notify(MULTIPLE_OUTPUTS);
        }
        let last_output = if let Some(lo) = out.all_output_texts().last() {
            lo
        } else {
            // No output at all.
            set_expected(&mut test_case, isbn, validated_isbn.is_ok());
            return Err(());
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
            test_case.notify(AMBIGUOUS_ANSWER);
        }
        set_expected(&mut test_case, isbn, validated_isbn.is_ok());

        Err(())
    };
    let result = result_call();

    match result {
        Ok(()) => {
            tests.add_success(test_case);
        }
        Err(()) => {
            tests.add_failure(test_case);
        }
    }
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
