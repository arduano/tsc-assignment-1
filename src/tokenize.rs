use std::str::FromStr;

// ====================
// Helper types
// ====================

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OperatorKind {
    Subtract,
    Add,
    Divide,
    Multiply,
}

// ====================
// Token
// ====================

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(OperatorKind),
    Number(f64),
}

// ====================
// State enum
// ====================

#[derive(Copy, Clone, PartialEq)]
enum State {
    Initial,
    WhitespaceBeforeOperator,
    NumberZeroInteger,
    NumberPoint,
    Number,
    End,
    Error,
}

// ====================
// Errors
// ====================

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumberLexingError {
    ExpectedDigitAfterPoint,
    NonZeroIntegerBeforePoint,
    MissingIntegerBeforePoint,
    ExpectedPointAfterZero,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExpressionLexingError {
    UnexpectedCharacter(char),
    ExpectedNumber,
    ExpectedOperator,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LexingError {
    IncorrectNumber(NumberLexingError),
    IncorrectExpression(ExpressionLexingError),
}

// ====================
// The lexer & implementation
// ====================

pub struct Lexer {
    buffer: Vec<char>,
    state: State,
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn get_operator_kind(c: char) -> Option<OperatorKind> {
    match c {
        '-' => Some(OperatorKind::Subtract),
        '+' => Some(OperatorKind::Add),
        '/' => Some(OperatorKind::Divide),
        '*' => Some(OperatorKind::Multiply),
        _ => None,
    }
}

impl Lexer {
    // Create a new lexer instance
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            state: State::Initial,
        }
    }

    pub fn is_ended(&self) -> bool {
        self.state == State::End
    }

    // Helper function
    // Drain all of the characters in self.buffer and convert it to a number.
    fn drain_buffer_to_decimal(&mut self) -> f64 {
        // Drain the characrers in the buffer and convert them to a string
        let string: String = self.buffer.drain(..).collect();

        // Convert the string to a decimal
        let number = f64::from_str(&string).unwrap();

        // Return the number token
        number
    }

    // Feed a character `Some(char)` to the lexer, or feed `None` for end of string.
    pub fn feed(&mut self, c: Option<char>) -> Result<Option<Vec<Token>>, LexingError> {
        // Process the remaining states
        match self.state {
            // If the state is end or error, return nothing
            State::End | State::Error => {
                return Ok(None);
            }

            // Initial state
            // Expect: digit, zero digit, whitespace
            State::Initial => {
                if let Some(c) = c {
                    // Not EOI

                    if is_digit(c) {
                        // == digit ==
                        // Push digit to the buffer, switch to the number (or zero number) state, return nothing
                        self.buffer.push(c);
                        if c == '0' {
                            self.state = State::NumberZeroInteger;
                        } else {
                            self.state = State::Number;
                        }
                        return Ok(None);
                    } else if c == ' ' {
                        // == whitespace ==
                        // Stay on the same state, return nothing
                        return Ok(None);
                    } else if let Some(_) = get_operator_kind(c) {
                        // !! error !!
                        // Unexpected operator
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::ExpectedNumber,
                        ));
                    } else if c == '.' {
                        // !! error !!
                        // Zero required before point
                        self.state = State::Error;
                        return Err(LexingError::IncorrectNumber(
                            NumberLexingError::MissingIntegerBeforePoint,
                        ));
                    } else {
                        // !! error !!
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                } else {
                    // !! error !!
                    // EOI not expected
                    self.state = State::Error;
                    return Err(LexingError::IncorrectExpression(
                        ExpressionLexingError::ExpectedNumber,
                    ));
                }
            }

            // Number (zero)
            // Expect: point, whitespace, operator, EOI
            State::NumberZeroInteger => {
                if let Some(c) = c {
                    if is_digit(c) {
                        // !! error !!
                        // Expected a decimal point after first zero
                        self.state = State::Error;
                        return Err(LexingError::IncorrectNumber(
                            NumberLexingError::ExpectedPointAfterZero,
                        ));
                    } else if c == '.' {
                        // == decimal point ==
                        // Push point to the buffer, switch to the point state, return nothing
                        self.buffer.push(c);
                        self.state = State::NumberPoint;
                        return Ok(None);
                    } else if c == ' ' {
                        // == whitespace ==
                        // Switch to first whitespace state, return number token
                        self.state = State::WhitespaceBeforeOperator;
                        return Ok(Some(vec![Token::Number(self.drain_buffer_to_decimal())]));
                    } else if let Some(operator_kind) = get_operator_kind(c) {
                        // == operator ==
                        // Switch to operator (initial) state, return number token and operator token
                        self.state = State::Initial;
                        return Ok(Some(vec![
                            Token::Number(self.drain_buffer_to_decimal()),
                            Token::Operator(operator_kind),
                        ]));
                    } else {
                        // !! error !!
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                } else {
                    // == EOI ==
                    // Switch to end state, return number token
                    self.state = State::End;
                    return Ok(Some(vec![Token::Number(self.drain_buffer_to_decimal())]));
                }
            }

            // Number (point)
            // Expect: digit
            State::NumberPoint => {
                if let Some(c) = c {
                    if is_digit(c) {
                        // == digit ==
                        // Push digit to the buffer, switch to the decimal state
                        self.buffer.push(c);
                        self.state = State::Number;
                        return Ok(None);
                    } else {
                        // !! error !!
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectNumber(
                            NumberLexingError::ExpectedDigitAfterPoint,
                        ));
                    }
                } else {
                    // !! error !!
                    // EOI not expected
                    self.state = State::Error;
                    return Err(LexingError::IncorrectNumber(
                        NumberLexingError::ExpectedDigitAfterPoint,
                    ));
                }
            }

            // Number (decimals)
            // Expect: digit, whitespace, operator, EOI
            State::Number => {
                if let Some(c) = c {
                    if is_digit(c) {
                        // == digit ==
                        // Push digit to the buffer, stay on the same state, return nothing
                        self.buffer.push(c);
                        return Ok(None);
                    } else if c == ' ' {
                        // == whitespace ==
                        // Switch to first whitespace state, return number token
                        self.state = State::WhitespaceBeforeOperator;
                        return Ok(Some(vec![Token::Number(self.drain_buffer_to_decimal())]));
                    } else if let Some(operator_kind) = get_operator_kind(c) {
                        // == operator ==
                        // Switch to operator state, return number token and operator token
                        self.state = State::Initial;
                        return Ok(Some(vec![
                            Token::Number(self.drain_buffer_to_decimal()),
                            Token::Operator(operator_kind),
                        ]));
                    } else if c == '.' {
                        // !! error !!
                        // Unexpected decimal point
                        self.state = State::Error;
                        return Err(LexingError::IncorrectNumber(
                            NumberLexingError::NonZeroIntegerBeforePoint,
                        ));
                    } else {
                        // !! error !!
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                } else {
                    // == EOI ==
                    // Switch to end state, return number token
                    self.state = State::End;
                    return Ok(Some(vec![Token::Number(self.drain_buffer_to_decimal())]));
                }
            }

            // First whitespace
            // Expect: whitespace, operator, EOI
            State::WhitespaceBeforeOperator => {
                if let Some(c) = c {
                    if c == ' ' {
                        // == whitespace ==
                        // Stay on the same state, return nothing
                        return Ok(None);
                    } else if let Some(operator_kind) = get_operator_kind(c) {
                        // == operator ==
                        // Switch to operator state, return nothing
                        self.state = State::Initial;
                        return Ok(Some(vec![Token::Operator(operator_kind)]));
                    } else if is_digit(c) {
                        // !! error !!
                        // Unexpected number
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::ExpectedOperator,
                        ));
                    } else {
                        // !! error !!
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                } else {
                    // == EOI ==
                    // Switch to end state, return nothing
                    self.state = State::End;
                    return Ok(None);
                }
            }
        }
    }
}

// ====================
// Get the token list for string
// ====================

pub fn tokenize(string: &str) -> Result<Vec<Token>, LexingError> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new();

    // Feed characters, one at a time
    for c in string.chars().into_iter() {
        let result = lexer.feed(Some(c))?;
        // If a token was emitted, add it to the list
        if let Some(mut token) = result {
            tokens.append(&mut token);
        }
    }

    // Feed EOI
    let result = lexer.feed(None)?;
    if let Some(mut token) = result {
        tokens.append(&mut token);
    }

    // Just in case, make sure the lexer is ended
    assert_eq!(lexer.is_ended(), true);

    Ok(tokens)
}
