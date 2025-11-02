pub type Isbn = i64;

#[derive(Debug, PartialEq)]
pub enum IsbnFailure {
    Length,
    Pattern,
}

pub fn is_correct_length(isbn: Isbn) -> bool {
    const LEN_14: i64 = 10_i64.pow(13); // 1 with 13 zeros: 14 digits
    const LEN_13: i64 = 10_i64.pow(12); // 1 with 12 zeros: 13 digits
    // important: LEN_13 is inclusive, LEN_14 exclusive
    (LEN_13..LEN_14).contains(&isbn)
}
pub fn is_correct_pattern(mut isbn: Isbn) -> bool {
    let mut digit_sum = 0;
    let mut weight = 1;
    while isbn != 0 {
        digit_sum += (isbn % 10) * weight;
        weight = if weight == 1 { 3 } else { 1 };
        isbn /= 10;
    }
    digit_sum % 10 == 0
}
pub fn check_isbn(isbn: Isbn) -> Result<(), IsbnFailure> {
    if !is_correct_length(isbn) {
        Err(IsbnFailure::Length)
    } else if !is_correct_pattern(isbn) {
        Err(IsbnFailure::Pattern)
    } else {
        Ok(())
    }
}

#[allow(unused)]
fn check_isbn_str(isbn: &str) -> Result<(), IsbnFailure> {
    if isbn.len() != 13 {
        Err(IsbnFailure::Length)
    } else if !is_correct_pattern_str(isbn) {
        Err(IsbnFailure::Pattern)
    } else {
        Ok(())
    }
}

fn is_correct_pattern_str(isbn: &str) -> bool {
    let mut digit_sum = 0;
    let mut weight = 1;
    for c in isbn.chars() {
        let digit = if let Some(d) = c.to_digit(10) {
            d
        } else {
            return false;
        };

        digit_sum += digit * weight;

        weight = if weight == 1 { 3 } else { 1 };
    }
    digit_sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::{check_isbn, check_isbn_str};
    use crate::a4::{
        CORRECT_LENGTH_WRONG_PATTERN, VALID_ISBN_INPUTS, WRONG_LENGTH_CORRECT_PATTERN,
        WRONG_LENGTH_WRONG_PATTERN,
    };

    #[test]
    fn isbn_str_and_number_impl_agree() {
        for input in CORRECT_LENGTH_WRONG_PATTERN
            .iter()
            .chain(&WRONG_LENGTH_WRONG_PATTERN)
            .chain(&WRONG_LENGTH_CORRECT_PATTERN)
            .chain(&VALID_ISBN_INPUTS)
        {
            let s = input.to_string();
            assert_eq!(
                check_isbn(*input).is_ok(),
                check_isbn_str(&s).is_ok(),
                "Implementations differ on {s}"
            );
        }
    }
}
