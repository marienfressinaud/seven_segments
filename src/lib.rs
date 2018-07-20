pub fn parse_number(string: &str) -> Result<i32, std::num::ParseIntError> {
    string.trim().parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_accepts_number() {
        let string_number = "42";
        let expected_number = 42;

        let resulting_number = parse_number(string_number).unwrap();

        assert_eq!(resulting_number, expected_number);
    }

    #[test]
    fn parse_number_accepts_surrounding_spaces() {
        let string_number = "   42  ";
        let expected_number = 42;

        let resulting_number = parse_number(string_number).unwrap();

        assert_eq!(resulting_number, expected_number);
    }

    #[test]
    fn parse_number_fails_when_passing_a_non_parsable_string() {
        let string_number = "foo42";

        let resulting_number = parse_number(string_number);

        assert!(
            resulting_number.is_err(),
            format!("{} is supposed to not be parsable", string_number)
        );
    }

    #[test]
    fn parse_number_fails_when_passing_an_empty_string() {
        let string_number = "";

        let resulting_number = parse_number(string_number);

        assert!(
            resulting_number.is_err(),
            "Empty string is supposed to not be parsable"
        );
    }
}
