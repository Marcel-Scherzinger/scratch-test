use super::isbn_logic::{IsbnFailure, check_isbn, is_correct_length, is_correct_pattern};
use super::*;

#[test]
fn valid_isbn_valid() {
    for isbn in VALID_ISBN_INPUTS {
        let res = check_isbn(isbn);
        assert_eq!(Ok(()), res, "{isbn} should be detected as valid");
        assert!(
            is_correct_length(isbn),
            "{isbn} should be detected as valid"
        );
        assert!(
            is_correct_pattern(isbn),
            "{isbn} should be detected as valid"
        );
    }
}
#[test]
fn invalid_isbn_pattern_detected() {
    for isbn in CORRECT_LENGTH_WRONG_PATTERN {
        let res = check_isbn(isbn);
        assert_eq!(
            Err(IsbnFailure::Pattern),
            res,
            "{isbn} should be detected as invalid"
        );
        assert!(
            is_correct_length(isbn),
            "{isbn} should be of correct length"
        );
        assert!(
            !is_correct_pattern(isbn),
            "{isbn} should have an invalid pattern"
        );
    }
}

#[test]
fn invalid_isbn_length_detected() {
    for isbn in WRONG_LENGTH_CORRECT_PATTERN {
        let res = check_isbn(isbn);
        assert_eq!(
            Err(IsbnFailure::Length),
            res,
            "{isbn} should be detected as invalid"
        );
        assert!(
            !is_correct_length(isbn),
            "{isbn} should have invalid length"
        );
        assert!(
            is_correct_pattern(isbn),
            "{isbn} should have a valid pattern"
        );
    }
}

#[test]
fn invalid_isbn_detected() {
    for isbn in WRONG_LENGTH_WRONG_PATTERN {
        let res = check_isbn(isbn);
        assert!(res.is_err(), "{isbn} should be detected as invalid");
        assert!(
            !is_correct_length(isbn),
            "{isbn} should have invalid length"
        );
        assert!(
            !is_correct_pattern(isbn),
            "{isbn} should have an invalid pattern"
        );
    }
}
