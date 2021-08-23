use std::str::FromStr;

// ====================
// Helper types
// ====================

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OperatorKind {
    Subtract,
    Plus,
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
    Whitespace,
}

// ====================
// State enums
// ====================

#[derive(Copy, Clone, PartialEq)]
enum State {
    Initial,
    WhitespaceBeforeOperator,
    WhitespaceAfterOperator,
    Operator(OperatorKind),
    NumberIntegers,
    NumberPoint,
    NumberDecimals,
    End,
    Error,
}

// ====================
// Errors
// ====================

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumberLexingError {
    ExpectedDigitAfterPoint,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExpressionLexingError {
    UnexpectedCharacter(char),
    UnexpectedEOI,
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
        '+' => Some(OperatorKind::Plus),
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
    pub fn feed(&mut self, c: Option<char>) -> Result<Option<Token>, LexingError> {
        // Process the remaining states
        match self.state {
            // If the state is end or error, return nothing
            State::End | State::Error => {
                return Ok(None);
            }

            // Initial state
            // Expect: digit
            State::Initial => {
                if let Some(c) = c {
                    // Not EOI

                    if is_digit(c) {
                        // == digit ==
                        // Push digit to the buffer, switch to the number state, return nothing
                        self.buffer.push(c);
                        self.state = State::NumberIntegers;
                        return Ok(None);
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
                        ExpressionLexingError::UnexpectedEOI,
                    ));
                }
            }

            // Number (integers)
            // Expect: digit, decimal point, whitespace, operator, EOI
            State::NumberIntegers => {
                if let Some(c) = c {
                    if is_digit(c) {
                        // == digit ==
                        // Push digit to the buffer, stay on the same state, return nothing
                        self.buffer.push(c);
                        return Ok(None);
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
                        return Ok(Some(Token::Number(self.drain_buffer_to_decimal())));
                    } else if let Some(operator_kind) = get_operator_kind(c) {
                        // == operator ==
                        // Switch to operator state, return number token
                        self.state = State::Operator(operator_kind);
                        return Ok(Some(Token::Number(self.drain_buffer_to_decimal())));
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
                    return Ok(Some(Token::Number(self.drain_buffer_to_decimal())));
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
                        self.state = State::NumberDecimals;
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
            State::NumberDecimals => {
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
                        return Ok(Some(Token::Number(self.drain_buffer_to_decimal())));
                    } else if let Some(operator_kind) = get_operator_kind(c) {
                        // == operator ==
                        // Switch to operator state, return number token
                        self.state = State::Operator(operator_kind);
                        return Ok(Some(Token::Number(self.drain_buffer_to_decimal())));
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
                    return Ok(Some(Token::Number(self.drain_buffer_to_decimal())));
                }
            }

            // First whitespace
            // Expect: whitespace, operator, EOI
            State::WhitespaceBeforeOperator => {
                if let Some(c) = c {
                    if c == ' ' {
                        // == whitespace ==
                        // Stay on the same state, return whitespace token
                        return Ok(Some(Token::Whitespace));
                    } else if let Some(operator_kind) = get_operator_kind(c) {
                        // == operator ==
                        // Switch to operator state, return whitespace token
                        self.state = State::Operator(operator_kind);
                        return Ok(Some(Token::Whitespace));
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
                    // Switch to end state, return whitespace token
                    self.state = State::End;
                    return Ok(Some(Token::Whitespace));
                }
            }

            // Operator
            // Expect: whitespace, digit
            State::Operator(operator_kind) => {
                if let Some(c) = c {
                    if c == ' ' {
                        // == whitespace ==
                        // Switch to second whitespace state, return operator token
                        self.state = State::WhitespaceAfterOperator;
                        return Ok(Some(Token::Operator(operator_kind)));
                    } else if is_digit(c) {
                        // == digit ==
                        // Switch to number state, return operator token
                        self.state = State::NumberIntegers;
                        self.buffer.push(c);
                        return Ok(Some(Token::Operator(operator_kind)));
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
                        ExpressionLexingError::UnexpectedEOI,
                    ));
                }
            }

            // Second whitespace
            // Expect: whitespace, digit
            State::WhitespaceAfterOperator => {
                if let Some(c) = c {
                    if c == ' ' {
                        // == whitespace ==
                        // Stay on the same state, return whitespace token
                        return Ok(Some(Token::Whitespace));
                    } else if is_digit(c) {
                        // == digit ==
                        // Push digit to buffer, switch to number state, return whitespace token
                        self.buffer.push(c);
                        self.state = State::NumberIntegers;
                        return Ok(Some(Token::Whitespace));
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
                        ExpressionLexingError::UnexpectedEOI,
                    ));
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
        if let Some(token) = result {
            tokens.push(token);
        }
    }

    // Feed EOI
    let result = lexer.feed(None)?;
    if let Some(token) = result {
        tokens.push(token);
    }

    // Just in case, make sure the lexer is ended
    assert_eq!(lexer.is_ended(), true);

    Ok(tokens)
}
