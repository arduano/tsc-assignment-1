use std::str::FromStr;

use rust_decimal::Decimal;

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
    Number(Decimal),
    Whitespace,
}

// ====================
// State enums
// ====================

#[derive(Copy, Clone, PartialEq)]
enum NumberState {
    Integers,
    Point,
    Decimals,
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Initial,
    WhitespaceBeforeOperator,
    WhitespaceAfterOperator,
    Operator(OperatorKind),
    FirstNumber(NumberState),
    SecondNumber(NumberState),
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

    // Feed a character `Some(char)` to the lexer, or feed `None` for end of string.
    pub fn feed(&mut self, c: Option<char>) -> Result<Option<Token>, LexingError> {
        // Progressing the number state
        match self.state {
            State::FirstNumber(number) | State::SecondNumber(number) => {
                // This is useful later when changing state
                let is_second_number = if let State::SecondNumber(..) = self.state {
                    true
                } else {
                    false
                };

                // Process the number state
                match number {
                    // Integers before the decimal
                    // Expect: digit, period, {other characters handled later}
                    NumberState::Integers => {
                        // Not EOI, EOI will be handled later
                        if let Some(c) = c {
                            if is_digit(c) {
                                // Push digit to the buffer, stay on the same state
                                self.buffer.push(c);
                                return Ok(None);
                            }
                            if c == '.' {
                                // Push point to the buffer, switch to the point state
                                self.buffer.push(c);

                                if is_second_number {
                                    self.state = State::SecondNumber(NumberState::Point);
                                } else {
                                    self.state = State::FirstNumber(NumberState::Point);
                                }

                                return Ok(None);
                            }
                        }
                    }

                    // Decimal point
                    // Expect: digit
                    NumberState::Point => {
                        if let Some(c) = c {
                            if is_digit(c) {
                                // Push digit to the buffer, switch to the decimal numbers state
                                self.buffer.push(c);

                                if is_second_number {
                                    self.state = State::SecondNumber(NumberState::Decimals);
                                } else {
                                    self.state = State::FirstNumber(NumberState::Decimals);
                                }

                                return Ok(None);
                            }
                        }

                        // If the input is not a digit, we have an error
                        return Err(LexingError::IncorrectNumber(
                            NumberLexingError::ExpectedDigitAfterPoint,
                        ));
                    }

                    // Decimal numbers
                    // Expect: digit, {other characters handled later}
                    NumberState::Decimals => {
                        // Not EOI, EOI will be handled later
                        if let Some(c) = c {
                            if is_digit(c) {
                                // Push digit to the buffer, stay on the same state
                                self.buffer.push(c);
                                return Ok(None);
                            }
                        }
                    }
                }
            }

            // Other states handled later
            _ => {}
        };

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
                        // Push digit to the buffer, switch to the first number state
                        self.buffer.push(c);
                        self.state = State::FirstNumber(NumberState::Integers);
                        return Ok(None);
                    } else {
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                } else {
                    // EOI not expected
                    self.state = State::Error;
                    return Err(LexingError::IncorrectExpression(
                        ExpressionLexingError::UnexpectedEOI,
                    ));
                }
            }

            // First number
            // - Expect: EOI, whitespace, operator
            // Second number
            // - Expect: EOI
            // Return number token at the end
            State::FirstNumber(..) | State::SecondNumber(..) => {
                if let Some(c) = c {
                    // Not EOI

                    // If second number, this is an error
                    if let State::SecondNumber(..) = self.state {
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }

                    // Is whitespace
                    if c == ' ' {
                        // Switch to whitespace state
                        self.state = State::WhitespaceBeforeOperator;
                    }
                    // Is operator
                    else if let Some(operator_kind) = get_operator_kind(c) {
                        // Switch to operator state
                        self.state = State::Operator(operator_kind);
                    } else {
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                } else {
                    // EOI
                    self.state = State::End;
                }

                // Drain the characrers in the buffer and convert them to a string
                let string: String = self.buffer.drain(..).collect();

                // Convert the string to a decimal
                let number = Decimal::from_str(&string).unwrap();

                // Return the number token
                return Ok(Some(Token::Number(number)));
            }

            // First whitespace
            // Expect: whitespace, operator
            State::WhitespaceBeforeOperator => {
                if let Some(c) = c {
                    // Not EOI
                    // Is whitespace
                    if c == ' ' {
                        // No change in state
                    }
                    // Is operator
                    else if let Some(operator_kind) = get_operator_kind(c) {
                        // Switch to operator state
                        self.state = State::Operator(operator_kind);
                    } else {
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }

                    // Return whitespace token
                    return Ok(Some(Token::Whitespace));
                } else {
                    // EOI not expected
                    self.state = State::Error;
                    return Err(LexingError::IncorrectExpression(
                        ExpressionLexingError::UnexpectedEOI,
                    ));
                }
            }

            // Operator
            // Expect: whitespace, digit
            State::Operator(operator_kind) => {
                if let Some(c) = c {
                    // Not EOI
                    if c == ' ' {
                        // Switch to whitespace state
                        self.state = State::WhitespaceAfterOperator;
                    }
                    // Is digit
                    else if is_digit(c) {
                        // Switch to digit state, push digit to buffer
                        self.buffer.push(c);
                        self.state = State::SecondNumber(NumberState::Integers);
                    } else {
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                    return Ok(Some(Token::Operator(operator_kind)));
                } else {
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
                    // Not EOI
                    // Is whitespace
                    if c == ' ' {
                        // No change in state
                    }
                    // Is digit
                    else if is_digit(c) {
                        // Switch to digit state, push digit to buffer
                        self.buffer.push(c);
                        self.state = State::SecondNumber(NumberState::Integers);
                    } else {
                        // Unexpected character
                        self.state = State::Error;
                        return Err(LexingError::IncorrectExpression(
                            ExpressionLexingError::UnexpectedCharacter(c),
                        ));
                    }
                    return Ok(Some(Token::Whitespace));
                } else {
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
