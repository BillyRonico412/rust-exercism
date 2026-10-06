use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub type Value = i32;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Manipulation {
    Dup,
    Drop,
    Swap,
    Over,
}
#[derive(PartialEq, Eq, Debug, Clone)]
enum Token {
    Number(Value),
    Operation(Operation),
    Manipulation(Manipulation),
    Call(usize),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Forth {
    stack: Vec<Value>,
    defs: Vec<Vec<Token>>,
    names: HashMap<String, usize>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    fn definition(&mut self, input: &str) -> Result<(), Error> {
        let mut splitted = input.split(" ");

        splitted
            .next()
            .filter(|&s| s == ":")
            .ok_or(Error::InvalidWord)?;

        let word = splitted
            .next()
            .and_then(|s| s.parse::<Value>().is_err().then_some(s.to_lowercase()))
            .ok_or(Error::InvalidWord)?;

        let def = splitted
            .take_while(|&s| s != ";")
            .map(|s| self.compute_token(s))
            .collect::<Result<Vec<Token>, Error>>()?;

        let i = self.defs.len();
        self.defs.push(def);
        self.names.insert(word, i);
        Ok(())
    }

    fn compute_token(&mut self, s: &str) -> Result<Token, Error> {
        let s = s.to_ascii_lowercase();
        let stack_item = match s.as_str() {
            _ if let Some(&def) = self.names.get(&s) => Token::Call(def),
            s if let Ok(s) = s.parse::<Value>() => Token::Number(s),
            "+" => Token::Operation(Operation::Plus),
            "-" => Token::Operation(Operation::Minus),
            "*" => Token::Operation(Operation::Mul),
            "/" => Token::Operation(Operation::Div),
            "dup" => Token::Manipulation(Manipulation::Dup),
            "drop" => Token::Manipulation(Manipulation::Drop),
            "swap" => Token::Manipulation(Manipulation::Swap),
            "over" => Token::Manipulation(Manipulation::Over),
            _ => return Err(Error::UnknownWord),
        };
        Ok(stack_item)
    }

    fn push(&mut self, stack_item: Token) -> Result<(), Error> {
        match stack_item {
            Token::Number(n) => {
                self.stack.push(n);
            }
            Token::Operation(op) => {
                let right = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let left = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let v = match op {
                    Operation::Plus => left + right,
                    Operation::Minus => left - right,
                    Operation::Mul => left * right,
                    Operation::Div => {
                        if right == 0 {
                            return Err(Error::DivisionByZero);
                        }
                        left / right
                    }
                };
                self.stack.push(v);
            }
            Token::Manipulation(manip) => match manip {
                Manipulation::Dup => {
                    let last = self.stack.last().ok_or(Error::StackUnderflow)?;
                    self.stack.push(*last);
                }
                Manipulation::Drop => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                Manipulation::Swap => {
                    let last = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let penultimate = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(last);
                    self.stack.push(penultimate);
                }
                Manipulation::Over => {
                    let last = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    let penultimate = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(penultimate);
                    self.stack.push(last);
                    self.stack.push(penultimate);
                }
            },
            Token::Call(index) => {
                let def = self.defs[index].iter().cloned().collect::<Vec<_>>();
                def.iter().try_for_each(|t| self.push(t.clone()))?;
            }
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        if input.starts_with(":") {
            return self.definition(input);
        }
        input.split(' ').try_for_each(|s| {
            let t = self.compute_token(s)?;
            self.push(t)
        })
    }
}
