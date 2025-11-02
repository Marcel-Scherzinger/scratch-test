pub fn parse_single_i64_number(text: &str) -> Option<i64> {
    enum State {
        Running(i64, i64),
        NotStarted,
        Finished(i64),
    }

    let mut state = State::NotStarted;
    for c in text.chars().rev() {
        state = match (c.to_digit(10), state) {
            // start number with first digit
            (Some(digit), State::NotStarted) => State::Running(digit.into(), 10),
            // continue number with another digit
            (Some(digit), State::Running(current, factor)) => {
                State::Running(digit as i64 * factor + current, factor * 10)
            }
            // found another number
            (Some(_), State::Finished(_)) => return None,
            // found a minus sign directly in front of the number
            (None, State::Running(num, _)) if c == '-' => State::Finished(-num),
            // found a non-digit character
            (None, State::Finished(num)) => State::Finished(num),
            // not yet started
            (None, State::NotStarted) => State::NotStarted,
            // finish because of non-digit
            (None, State::Running(num, _)) => State::Finished(num),
        }
    }
    match state {
        State::NotStarted => None,
        State::Running(n, _) | State::Finished(n) => Some(n),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_single_i64_number as parse;

    #[test]
    fn test_single_positive() {
        assert_eq!(Some(1234), parse("1234"));
        assert_eq!(Some(42), parse("something 42 around it"));
        assert_eq!(Some(3333), parse("just: (3333)"));
    }

    #[test]
    fn test_multiple_positives() {
        assert_eq!(None, parse("multiple 1, 2, 3"));
        assert_eq!(None, parse("multiple 1 3"));
    }

    #[test]
    fn test_negative() {
        assert_eq!(Some(-42), parse("-42"));
        assert_eq!(Some(-42), parse("negative-42"));
    }
}
