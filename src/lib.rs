pub fn parse_number(string: &str) -> Result<i32, std::num::ParseIntError> {
    string.trim().parse()
}

pub fn split_into_digits(number: i32) -> Vec<u8> {
    if number == 0 {
        return vec![0];
    }

    let mut digits: Vec<u8> = Vec::new();
    let mut number = number;
    while number != 0 {
        digits.insert(0, (number % 10) as u8);
        number = number / 10;
    }
    digits
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const DIGIT_PATTERNS: &'static [&'static [&'static str]] = &[
    &[" _ ", "   ", " _ ", " _ ", "   ", " _ ", " _ ", " _ ", " _ ", " _ "],
    &["| |", "  |", " _|", " _|", "|_|", "|_ ", "|_ ", "  |", "|_|", "|_|"],
    &["|_|", "  |", "|_ ", " _|", "  |", " _|", "|_|", "  |", "|_|", " _|"],
];

pub fn render_digits(digits: &Vec<u8>, output: &mut String) {
    for position in 0..=2 {
        let rendered_position = digits
            .iter()
            .map(|digit| DIGIT_PATTERNS[position][*digit as usize])
            .collect::<Vec<&'static str>>()
            .join(" ");
        output.push_str(&format!("{}\n", rendered_position));
    }
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

    #[test]
    fn split_into_digits_returns_vec_of_digits() {
        let number = 42;
        let expected_digits = [4, 2];

        let resulting_digits = split_into_digits(number);

        assert_eq!(resulting_digits, expected_digits);
    }

    #[test]
    fn split_into_digits_accepts_zero_value() {
        let number = 0;
        let expected_digits = [0];

        let resulting_digits = split_into_digits(number);

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

        let mut resulting_output = String::new();
        render_digits(&digits, &mut resulting_output);

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

        let mut resulting_output = String::new();
        render_digits(&digits, &mut resulting_output);

        assert_eq!(resulting_output, expected_output);
    }
}
