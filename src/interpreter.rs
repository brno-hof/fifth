use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Push(u8),
    Pop,
    Dup,
    Swap,
    Rotate,
    Over,
    Pick(usize),
    BinOp(BinOp),
    PrintByte,
    PrintChar,
    If,
    Else,
    Then,
    Call(String),
    Return,
    Halt,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Push(n) => format!("push {}", n),
            Token::Pop => "pop".to_string(),
            Token::Dup => "dup".to_string(),
            Token::Swap => "swap".to_string(),
            Token::Rotate => "rotate".to_string(),
            Token::Over => "over".to_string(),
            Token::Pick(n) => format!("pick {}", n),
            Token::BinOp(op) => match op {
                BinOp::Add => "add".to_string(),
                BinOp::Sub => "sub".to_string(),
            },
            Token::PrintByte => "print_byte".to_string(),
            Token::PrintChar => "print_char".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Then => "then".to_string(),
            Token::Call(label) => label.to_lowercase(),
            Token::Return => "return".to_string(),
            Token::Halt => "halt".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub struct AnnotatedToken {
    pub token: Token,
    pub line_number: usize,
}

#[derive(Debug)]
pub enum RuntimeError {
    StackOverflow(AnnotatedToken),
    StackUnderflow(AnnotatedToken),
    InvalidLabel(AnnotatedToken),
    CallStackUnderflow(AnnotatedToken),
    UnclosedIfStatement(AnnotatedToken),
}

#[derive(Debug)]
pub enum ParseError {
    InvalidArgument(String, usize),
    MissingArgument(String, usize),
    DuplicateLabel(String, usize),
    InvalidCall(String, usize),
    ElseWithoutIfStatement(AnnotatedToken),
    ThenWithoutIfStatement(AnnotatedToken),
    TooManyElseStatements(AnnotatedToken),
}

pub struct Program {
    pub lines: Vec<String>,
    pub tokens: Vec<AnnotatedToken>,
    pub pc: usize,
    labels: HashMap<String, usize>,
    call_stack: Vec<usize>,
    pub stack: Vec<u8>,
    pub stack_size: usize,
    pub halted: bool,
}

impl Program {
    pub fn new(text: &str, stack_size: usize) -> Self {
        let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
        Self {
            lines,
            tokens: Vec::new(),
            pc: 0,
            labels: HashMap::new(),
            call_stack: Vec::new(),
            stack: Vec::with_capacity(stack_size),
            stack_size,
            halted: false,
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        for (line_number, line) in (1..).zip(self.lines.iter()) {
            let mut parts = line.split_whitespace();
            if let Some(part) = parts.next() {
                if part.starts_with('#') {
                    continue;
                }
                if part.ends_with(":") {
                    match self
                        .labels
                        .entry(part[..part.len() - 1].to_uppercase().to_string())
                    {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(self.tokens.len());
                        }
                        std::collections::hash_map::Entry::Occupied(_) => {
                            return Err(ParseError::DuplicateLabel(part.to_string(), line_number))
                        }
                    }
                    continue;
                };
                let token = match part.to_uppercase().as_str() {
                    "PUSH" => match parts.next() {
                        None => {
                            return Err(ParseError::MissingArgument(part.to_string(), line_number))
                        }
                        Some(arg) => match arg.parse::<u8>() {
                            Ok(value) => Token::Push(value),
                            Err(_) => {
                                return Err(ParseError::InvalidArgument(
                                    arg.to_string(),
                                    line_number,
                                ))
                            }
                        },
                    },
                    "POP" => Token::Pop,
                    "DUP" => Token::Dup,
                    "SWAP" => Token::Swap,
                    "OVER" => Token::Over,
                    "ROTATE" => Token::Rotate,
                    "PICK" => match parts.next() {
                        None => {
                            return Err(ParseError::MissingArgument(part.to_string(), line_number))
                        }
                        Some(arg) => match arg.parse::<usize>() {
                            Ok(value) => Token::Pick(value),
                            Err(_) => {
                                return Err(ParseError::InvalidArgument(
                                    arg.to_string(),
                                    line_number,
                                ))
                            }
                        },
                    },
                    "ADD" => Token::BinOp(BinOp::Add),
                    "SUB" => Token::BinOp(BinOp::Sub),
                    "PRINT_BYTE" => Token::PrintByte,
                    "PRINT_CHAR" => Token::PrintChar,
                    "IF" => Token::If,
                    "ELSE" => Token::Else,
                    "THEN" => Token::Then,
                    "RETURN" => Token::Return,
                    "HALT" => Token::Halt,
                    other => Token::Call(other.to_string()),
                };
                self.tokens.push(AnnotatedToken { token, line_number })
            }
        }
        if let Err(parse_error) = self.check_if_statements() {
            return Err(parse_error);
        };
        if let Err(parse_error) = self.check_calls() {
            return Err(parse_error);
        };
        Ok(())
    }

    fn check_calls(&self) -> Result<(), ParseError> {
        for annotated_token in &self.tokens {
            if let Token::Call(label) = &annotated_token.token {
                if let None = self.labels.get(label) {
                    return Err(ParseError::InvalidCall(
                        label.to_string(),
                        annotated_token.line_number,
                    ));
                }
            }
        }
        Ok(())
    }

    fn check_if_statements(&self) -> Result<(), ParseError> {
        let mut else_statements: Vec<u32> = Vec::new();
        for annotated_token in &self.tokens {
            match annotated_token.token {
                Token::If => {
                    else_statements.push(0);
                }
                Token::Else => {
                    let num_else_statements_at_depth = match else_statements.pop() {
                        None => {
                            return Err(ParseError::ElseWithoutIfStatement(annotated_token.clone()))
                        }
                        Some(n) => n,
                    };
                    if num_else_statements_at_depth > 0 {
                        return Err(ParseError::TooManyElseStatements(annotated_token.clone()));
                    }
                    else_statements.push(num_else_statements_at_depth + 1);
                }
                Token::Then => {
                    if let None = else_statements.pop() {
                        return Err(ParseError::ThenWithoutIfStatement(annotated_token.clone()));
                    };
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        if self.pc >= self.tokens.len() || self.halted {
            return Ok(());
        }
        let current_token = &self.tokens[self.pc];

        match &current_token.token {
            Token::Push(value) => {
                if self.stack.len() < self.stack_size {
                    self.pc += 1;
                    self.stack.push(value.clone());
                } else {
                    return Err(RuntimeError::StackOverflow(current_token.clone()));
                }
            }
            Token::Pop => match self.stack.pop() {
                None => return Err(RuntimeError::StackUnderflow(current_token.clone())),
                Some(_) => {
                    self.pc += 1;
                }
            },
            Token::Dup => match self.stack.last() {
                None => return Err(RuntimeError::StackUnderflow(current_token.clone())),
                Some(&top) => {
                    self.stack.push(top);
                    self.pc += 1;
                }
            },
            Token::Swap => match (self.stack.pop(), self.stack.pop()) {
                (None, _) | (_, None) => {
                    return Err(RuntimeError::StackUnderflow(current_token.clone()))
                }
                (Some(top), Some(bottom)) => {
                    self.stack.push(top);
                    self.stack.push(bottom);
                    self.pc += 1;
                }
            },
            Token::Over => match self.stack.last_chunk::<2>() {
                None => return Err(RuntimeError::StackUnderflow(current_token.clone())),
                Some(last_two) => {
                    self.stack.push(last_two[0]);
                    self.pc += 1;
                }
            },
            Token::Rotate => match (self.stack.pop(), self.stack.pop(), self.stack.pop()) {
                (None, _, _) | (_, None, _) | (_, _, None) => {
                    return Err(RuntimeError::StackUnderflow(current_token.clone()))
                }
                (Some(top), Some(middle), Some(bottom)) => {
                    self.stack.push(middle);
                    self.stack.push(top);
                    self.stack.push(bottom);
                    self.pc += 1;
                }
            },
            Token::Pick(index) => {
                let value = match self.stack.get(self.stack.len() - 1 - index) {
                    None => return Err(RuntimeError::StackUnderflow(current_token.clone())),
                    Some(&value) => value,
                };
                self.stack.push(value);
                self.pc += 1;
            }
            Token::BinOp(bin_op) => match (self.stack.pop(), self.stack.pop()) {
                (None, _) | (_, None) => {
                    return Err(RuntimeError::StackUnderflow(current_token.clone()))
                }
                (Some(top), Some(bottom)) => {
                    let result = match bin_op {
                        BinOp::Add => top.overflowing_add(bottom).0,
                        BinOp::Sub => bottom.overflowing_sub(top).0,
                    };
                    self.stack.push(result);
                    self.pc += 1;
                }
            },
            Token::PrintByte | Token::PrintChar => match self.stack.pop() {
                None => return Err(RuntimeError::StackUnderflow(current_token.clone())),
                Some(top) => {
                    if let Token::PrintByte = &current_token.token {
                        print!("{}", top);
                    };
                    if let Token::PrintChar = &current_token.token {
                        let character = char::from(top);
                        print!("{}", character);
                    }
                    self.pc += 1;
                }
            },
            Token::If => {
                let top = match self.stack.last() {
                    Some(&top) => top,
                    None => return Err(RuntimeError::StackUnderflow(current_token.clone())),
                };

                if top > 0 {
                    self.pc += 1;
                } else {
                    let mut depth = 1;
                    let mut found_else = false;
                    let mut found_then = false;
                    while !(depth == 0 && found_then || depth == 1 && found_else) {
                        self.pc += 1;
                        if self.pc >= self.tokens.len() {
                            return Err(RuntimeError::UnclosedIfStatement(current_token.clone()));
                        }
                        found_else = false;
                        found_then = false;
                        match self.tokens[self.pc].token {
                            Token::If => {
                                depth += 1;
                            }
                            Token::Else => {
                                found_else = true;
                            }
                            Token::Then => {
                                found_then = true;
                                depth -= 1;
                            }
                            _ => (),
                        }
                    }
                    self.pc += 1;
                }
            }
            Token::Else => {
                let mut depth = 1;
                let mut found_then = false;
                while !(depth == 0 && found_then) {
                    self.pc += 1;
                    if self.pc >= self.tokens.len() {
                        return Err(RuntimeError::UnclosedIfStatement(current_token.clone()));
                    }
                    found_then = false;
                    match self.tokens[self.pc].token {
                        Token::If => {
                            depth += 1;
                        }
                        Token::Then => {
                            found_then = true;
                            depth -= 1;
                        }
                        _ => (),
                    }
                }
            }
            Token::Then => {
                self.pc += 1;
            }
            Token::Call(label) => match self.labels.get(label) {
                None => return Err(RuntimeError::InvalidLabel(current_token.clone())),
                Some(index) => {
                    self.call_stack.push(self.pc + 1);
                    self.pc = index.clone();
                }
            },
            Token::Return => {
                self.pc = match self.call_stack.pop() {
                    Some(index) => index,
                    None => return Err(RuntimeError::CallStackUnderflow(current_token.clone())),
                };
            }
            Token::Halt => {
                self.halted = true;
            }
        };
        Ok(())
    }

    pub fn _stack(&self) -> &[u8] {
        &self.stack
    }

    pub fn _lines_as_string(&self) -> String {
        format!("{:?}", &self.lines)
    }

    pub fn _tokens_as_string(&self) -> String {
        format!("{:?}", &self.tokens)
    }

    pub fn _stack_as_string(&self) -> String {
        format!("{:?}", &self.stack)
    }
}
