use task::converter_module::*;

#[tokio::test]
async fn test_display_currencies() {
    let mut currencies: std::collections::HashMap<String, std::collections::HashMap<String, f64>> = std::collections::HashMap::new();
    assert!(display_currencies(&mut currencies).await.is_ok());
}

#[test]
fn test_is_uppercase() {
    assert!(is_uppercase("ABC"));
    assert!(!is_uppercase("abc"));
    assert!(!is_uppercase("AbC"));
}

#[test]
fn test_validate_read_value_positive() {
    let input = "42.5".to_string();
    let result = validate_read_value(input);
    assert_eq!(result, 42.5);
}

#[test]
fn test_validate_read_value_negative() {
    let input = "-5.0".to_string();
    let result = validate_read_value(input);
    assert_eq!(result, -1.0);
}