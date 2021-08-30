use rand::Rng;

use crate::tokenize::{
    tokenize, ExpressionLexingError, LexingError, NumberLexingError, OperatorKind, Token,
};

/// The number of times to repeat each test, for more stability
const REPEAT_COUNT: i32 = 1000;

/// If something went wrong with generating the test cases
macro_rules! bad_test {
    () => {
        panic!("Unknown error in the test (Please contact course coordinator)");
    };
}

fn assert_eq_with_input<T: std::fmt::Debug + PartialEq>(input: &str, expected: &T, actual: &T) {
    assert_eq!(expected, actual, "\n input: \"{}\"\n\n", input);
}

fn digit_to_char(digit: i32) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => bad_test!(),
    }
}

fn random_non_zero_digit() -> char {
    let mut rng = rand::thread_rng();
    let digit = rng.gen_range(1..10);
    digit_to_char(digit)
}

fn random_digit() -> char {
    let mut rng = rand::thread_rng();
    let digit = rng.gen_range(0..10);
    digit_to_char(digit)
}

fn random_integer_string(len: Option<i32>) -> String {
    let len = if let Some(len) = len {
        len
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..5)
    };

    (0..len)
        .map(|i| {
            if i == 0 {
                random_non_zero_digit()
            } else {
                random_digit()
            }
        })
        .collect()
}

fn random_decimal_string(len: Option<i32>) -> String {
    format!("0.{}", random_integer_string(len))
}

fn random_number_token() -> Token {
    let mut rng = rand::thread_rng();
    let is_decimal = rng.gen_bool(0.5);

    if is_decimal {
        as_number_token(&random_decimal_string(None))
    } else {
        as_number_token(&random_integer_string(None))
    }
}

fn random_operator() -> OperatorKind {
    let mut rng = rand::thread_rng();
    let operator = rng.gen_range(0..4);
    match operator {
        0 => OperatorKind::Subtract,
        1 => OperatorKind::Add,
        2 => OperatorKind::Divide,
        3 => OperatorKind::Multiply,
        _ => bad_test!(),
    }
}

fn random_operator_token() -> Token {
    Token::Operator(random_operator())
}

fn as_number_token(str: &str) -> Token {
    Token::Number(str.parse::<f64>().unwrap())
}

fn token_to_string(token: &Token) -> String {
    match token {
        Token::Number(num) => num.to_string(),
        Token::Operator(op) => match op {
            OperatorKind::Add => "+".to_string(),
            OperatorKind::Subtract => "-".to_string(),
            OperatorKind::Multiply => "*".to_string(),
            OperatorKind::Divide => "/".to_string(),
        },
    }
}

fn pad_with_random_whitespaces(strings: &mut Vec<String>) {
    if strings.len() > 1 {
        let mut rng = rand::thread_rng();
        for _ in 0..(strings.len() - 1) {
            let pos = rng.gen_range(1..strings.len());
            strings.insert(pos, " ".to_string());
        }
    }
}

fn token_list_to_string_list(tokens: &Vec<Token>) -> Vec<String> {
    tokens.iter().map(|t| token_to_string(t)).collect()
}

fn token_list_to_string(tokens: &Vec<Token>) -> String {
    let mut strings: Vec<String> = token_list_to_string_list(tokens);

    pad_with_random_whitespaces(&mut strings);

    strings.join("")
}

fn random_valid_token_sequence(operator_count: Option<i32>) -> Vec<Token> {
    let operator_count = if let Some(operator_count) = operator_count {
        operator_count
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..5)
    };

    let mut tokens = vec![];
    for i in 0..operator_count {
        if i != 0 {
            tokens.push(random_operator_token());
        }
        tokens.push(random_number_token());
    }

    tokens
}

fn random_even_number_under(mut before: usize) -> usize {
    let mut rng = rand::thread_rng();
    if before % 2 == 1 {
        before += 1;
    }
    before /= 2;
    let number = rng.gen_range(0..before);
    number * 2
}

fn random_valid_token_sequence_with_replaced_number(replacement: String) -> String {
    let seq = random_valid_token_sequence(None);
    let mut strings = token_list_to_string_list(&seq);

    let index = random_even_number_under(strings.len());
    strings[index] = replacement;

    pad_with_random_whitespaces(&mut strings);
    let string = strings.join("");

    string
}

/// Helper function to repeat a test multiple times
/// Students have had issues with tests sometimes passing and sometimes failing
/// This function will help to make sure that the result is more stable
fn repeat(test: impl Fn()) {
    for _ in 0..REPEAT_COUNT {
        test();
    }
}

#[test]
fn test_analyze_single_digit_integer() {
    repeat(|| {
        let number = random_integer_string(Some(1));
        let tokens = vec![as_number_token(&number)];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_analyze_multi_digit_integer() {
    repeat(|| {
        let number = random_integer_string(None);
        let tokens = vec![as_number_token(&number)];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_analyze_single_digit_decimal() {
    repeat(|| {
        let number = random_decimal_string(Some(1));
        let tokens = vec![as_number_token(&number)];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_analyze_multi_digit_decimal() {
    repeat(|| {
        let number = random_decimal_string(None);
        let tokens = vec![as_number_token(&number)];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_non_zero_integer_part() {
    repeat(|| {
        let invalid_number = format!(
            "{}.{}",
            random_integer_string(None),
            random_integer_string(None)
        );
        let string = random_valid_token_sequence_with_replaced_number(invalid_number);

        let output = tokenize(&string);

        assert_eq_with_input(
            &string,
            &output,
            &Err(LexingError::IncorrectNumber(
                NumberLexingError::NonZeroIntegerBeforePoint,
            )),
        );
    })
}

#[test]
fn test_decimal_without_integer_part() {
    repeat(|| {
        let invalid_number = format!(".{}", random_integer_string(None));
        let string = random_valid_token_sequence_with_replaced_number(invalid_number);

        let output = tokenize(&string);

        assert_eq_with_input(
            &string,
            &output,
            &Err(LexingError::IncorrectNumber(
                NumberLexingError::MissingIntegerBeforePoint,
            )),
        );
    })
}

#[test]
fn test_integer_starting_with_zero() {
    repeat(|| {
        let invalid_number = format!("0{}", random_integer_string(None));
        let string = random_valid_token_sequence_with_replaced_number(invalid_number);

        let output = tokenize(&string);

        assert_eq_with_input(
            &string,
            &output,
            &Err(LexingError::IncorrectNumber(
                NumberLexingError::ExpectedPointAfterZero,
            )),
        );
    })
}

#[test]
fn test_decimal_without_decimal_part() {
    repeat(|| {
        let invalid_number = "0.".to_string();
        let string = random_valid_token_sequence_with_replaced_number(invalid_number);

        let output = tokenize(&string);

        assert_eq_with_input(
            &string,
            &output,
            &Err(LexingError::IncorrectNumber(
                NumberLexingError::ExpectedDigitAfterPoint,
            )),
        );
    })
}

#[test]
fn test_simple_plus() {
    repeat(|| {
        let tokens = vec![
            random_number_token(),
            Token::Operator(OperatorKind::Add),
            random_number_token(),
        ];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_simple_minus() {
    repeat(|| {
        let tokens = vec![
            random_number_token(),
            Token::Operator(OperatorKind::Subtract),
            random_number_token(),
        ];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_simple_multiply() {
    repeat(|| {
        let tokens = vec![
            random_number_token(),
            Token::Operator(OperatorKind::Multiply),
            random_number_token(),
        ];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_simple_divide() {
    repeat(|| {
        let tokens = vec![
            random_number_token(),
            Token::Operator(OperatorKind::Divide),
            random_number_token(),
        ];
        let string = token_list_to_string(&tokens);

        let output = tokenize(&string);

        assert_eq_with_input(&string, &output, &Ok(tokens));
    })
}

#[test]
fn test_expected_number_error() {
    let strings = vec!["1+", " 1 - ", "", " / 5 ", "* 5 "];

    for string in strings.into_iter() {
        let output = tokenize(string);

        assert_eq_with_input(
            string,
            &output,
            &Err(LexingError::IncorrectExpression(
                ExpressionLexingError::ExpectedNumber,
            )),
        );
    }
}

#[test]
fn test_expected_operator_error() {
    let strings = vec!["1 0.2", " 0.3 2"];

    for string in strings.into_iter() {
        let output = tokenize(string);

        assert_eq_with_input(
            string,
            &output,
            &Err(LexingError::IncorrectExpression(
                ExpressionLexingError::ExpectedOperator,
            )),
        );
    }
}
