use crate::tokenize::{
    tokenize, ExpressionLexingError, LexingError, NumberLexingError, OperatorKind, Token,
};

// ===================
// Success tests
// ===================

#[test]
fn parses_correctly() {
    let tokens = tokenize("16.2400 + 00-2.1 / 5").unwrap();

    let expected_tokens = vec![
        Token::Number(16.24),
        Token::Whitespace,
        Token::Operator(OperatorKind::Plus),
        Token::Whitespace,
        Token::Number(0.0),
        Token::Operator(OperatorKind::Subtract),
        Token::Number(2.1),
        Token::Whitespace,
        Token::Operator(OperatorKind::Divide),
        Token::Whitespace,
        Token::Number(5.0),
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn parses_without_whitespace() {
    let tokens = tokenize("0016/0.5").unwrap();

    let expected_tokens = vec![
        Token::Number(16.0),
        Token::Operator(OperatorKind::Divide),
        Token::Number(0.5),
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn parses_single_number() {
    let tokens = tokenize("00.050").unwrap();

    let expected_tokens = vec![Token::Number(0.05)];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn parses_irregular_whitespaces_1() {
    let tokens = tokenize("3463    *2/3463.0-   2.0").unwrap();

    let expected_tokens = vec![
        Token::Number(3463.0),
        Token::Whitespace,
        Token::Whitespace,
        Token::Whitespace,
        Token::Whitespace,
        Token::Operator(OperatorKind::Multiply),
        Token::Number(2.0),
        Token::Operator(OperatorKind::Divide),
        Token::Number(3463.0),
        Token::Operator(OperatorKind::Subtract),
        Token::Whitespace,
        Token::Whitespace,
        Token::Whitespace,
        Token::Number(2.0),
    ];
    assert_eq!(tokens, expected_tokens);
}

// ===================
// Error tests
// ===================

#[test]
fn errors_on_missing_decimals() {
    let tests = vec!["43.", "43..464", "43. + 96", "43.0 + 96."];

    for test in tests {
        let err = tokenize(test).err();

        let expected_error =
            LexingError::IncorrectNumber(NumberLexingError::ExpectedDigitAfterPoint);

        assert_eq!(err, Some(expected_error));
    }
}

#[test]
fn errors_on_eoi() {
    let tests = vec!["43.0   /", "43 + 43- ", "43 +  "];

    for test in tests {
        let err = tokenize(test).err();

        let expected_error = LexingError::IncorrectExpression(ExpressionLexingError::UnexpectedEOI);

        assert_eq!(err, Some(expected_error));
    }
}

#[test]
fn errors_on_unexpected_char() {
    let tests = vec![
        ("43.0   dfgd ", 'd'),
        ("43  f  ", 'f'),
        ("43 + + 82", '+'),
        ("43 - 3 - 43 .", '.'),
        ("76.5.344 ", '.'),
    ];

    for test in tests {
        let err = tokenize(test.0).err();

        let expected_error =
            LexingError::IncorrectExpression(ExpressionLexingError::UnexpectedCharacter(test.1));

        assert_eq!(err, Some(expected_error));
    }
}
