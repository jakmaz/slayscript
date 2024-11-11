pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
    let digits = &s[..end];
    let remaining = &s[end..];
    (remaining, digits)
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
}
