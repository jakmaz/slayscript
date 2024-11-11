pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    if s.starts_with("drip") {
        (&s[4..], "drip")
    } else if s.starts_with("lack") {
        (&s[4..], "lack")
    } else if s.starts_with("combo") {
        (&s[5..], "combo")
    } else if s.starts_with("ratio") {
        (&s[5..], "ratio")
    } else {
        panic!("bad operator");
    }
}

mod test {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10lack20"), ("lack20", "10"));
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("drip2"), ("2", "drip"));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("lack10"), ("10", "lack"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_op("combo3"), ("3", "combo"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("ratio4"), ("4", "ratio"));
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }
}
