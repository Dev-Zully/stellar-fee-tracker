//! Malformed-input edge cases for parser/validator/ledger utilities.
fn parse_positive_i64(raw: &str) -> Result<i64, String> {
    raw.parse::<i64>()
        .map_err(|_| format!("not an integer: {raw}"))
        .and_then(|v| if v >= 0 { Ok(v) } else { Err("must be non-negative".into()) })
}

#[test]
fn rejects_non_numeric_input() {
    assert!(parse_positive_i64("not-a-number").is_err());
}

#[test]
fn rejects_negative_values() {
    assert!(parse_positive_i64("-5").is_err());
}

#[test]
fn accepts_valid_non_negative_integer() {
    assert_eq!(parse_positive_i64("42"), Ok(42));
}
