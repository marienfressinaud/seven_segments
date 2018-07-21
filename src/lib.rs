pub fn parse_digits(string: &str) -> Result<Vec<u8>, &'static str> {
    let mut digits: Vec<u8> = Vec::new();
    for c in string.trim().chars() {
        match c.to_digit(10) {
            Some(digit) => digits.push(digit as u8),
            None => return Err("parameter cannot be parsed as digits"),
        };
    }
    Ok(digits)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const DIGIT_PATTERNS: &'static [&'static [&'static str]] = &[
    &[" _ ", "   ", " _ ", " _ ", "   ", " _ ", " _ ", " _ ", " _ ", " _ "],
    &["| |", "  |", " _|", " _|", "|_|", "|_ ", "|_ ", "  |", "|_|", "|_|"],
    &["|_|", "  |", "|_ ", " _|", "  |", " _|", "|_|", "  |", "|_|", " _|"],
];

pub fn render_digits(digits: &Vec<u8>) -> String {
    let mut output = String::new();
    for position in 0..=2 {
        let rendered_position = digits
            .iter()
            .map(|digit| DIGIT_PATTERNS[position][*digit as usize])
            .collect::<Vec<&'static str>>()
            .join(" ");
        output.push_str(&format!("{}\n", rendered_position));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits_accepts_number() {
        let string_number = "42";
        let expected_digits = [4, 2];

        let resulting_digits = parse_digits(string_number).unwrap();

        assert_eq!(resulting_digits, expected_digits);
    }

    #[test]
    fn parse_digits_accepts_surrounding_spaces() {
        let string_number = "   42  ";
        let expected_digits = [4, 2];

        let resulting_digits = parse_digits(string_number).unwrap();

        assert_eq!(resulting_digits, expected_digits);
    }

    #[test]
    fn parse_digits_fails_when_passing_a_non_parsable_string() {
        let string_number = "foo42";

        let resulting_digits = parse_digits(string_number);

        assert!(
            resulting_digits.is_err(),
            format!("{} is supposed to not be parsable", string_number)
        );
    }

    #[test]
    fn parse_digits_accepts_empty_strings() {
        let string_number = "";
        let expected_digits = [];

        let resulting_digits = parse_digits(string_number).unwrap();

        assert_eq!(resulting_digits, expected_digits);
    }

    #[test]
    fn parse_digits_accepts_big_numbers() {
        let string_number = "184467440737095516159";
        let expected_digits = [
            1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5, 9,
        ];

        let resulting_digits = parse_digits(string_number).unwrap();

        assert_eq!(resulting_digits, expected_digits);
    }

    #[test]
    fn parse_digits_accepts_leading_zeros() {
        let string_number = "0042";
        let expected_digits = [0, 0, 4, 2];

        let resulting_digits = parse_digits(string_number).unwrap();

        assert_eq!(resulting_digits, expected_digits);
    }

    #[test]
    fn render_digits_returns_string_representing_7_segments_output() {
        let digits = vec![3];
        let expected_output = r#"
 _ 
 _|
 _|
"#.trim_left_matches('\n');

        let resulting_output = render_digits(&digits);

        assert_eq!(resulting_output, expected_output);
    }

    #[test]
    fn render_digits_returns_digits_seperated_by_spaces() {
        let digits = vec![4, 2];
        let expected_output = r#"
     _ 
|_|  _|
  | |_ 
"#.trim_left_matches('\n');

        let resulting_output = render_digits(&digits);

        assert_eq!(resulting_output, expected_output);
    }
}
