use crate::{escape_ident, is_ident};

#[test]
fn escape() {
    assert_eq!(escape_ident("true"), "r#true");
    assert_eq!(escape_ident("not_a_keyword"), "not_a_keyword");
}

#[test]
fn test_is_ident() {
    assert!(is_ident("r#true"));
    assert!(is_ident("r#not_a_keyword"));
    assert!(!is_ident("r#_"));
    assert!(!is_ident("_"));
    assert!(is_ident("a"));
    assert!(is_ident("_a"));
    assert!(!is_ident("0"));
    assert!(!is_ident("true"));
}
