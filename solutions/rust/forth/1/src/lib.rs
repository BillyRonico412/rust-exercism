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
enum StackItem {
    Number(Value),
    Operation(Operation),
    Manipulation(Manipulation),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Forth {
    stack: Vec<Value>,
    definitions: HashMap<String, String>,
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
            .is_some_and(|s| s == ":")
            .ok_or(Error::InvalidWord)?;

        let word = splitted
            .next()
            .and_then(|s| s.parse::<Value>().is_err().then_some(s.to_lowercase()))
            .ok_or(Error::InvalidWord)?;

        let stack_items = splitted
            .take_while(|&s| s != ";")
            .map(|s| self.definitions.get(s).cloned().unwrap_or(s.to_lowercase()))
            .collect::<Vec<_>>()
            .join(" ");

        self.definitions.insert(word, stack_items);

        Ok(())
    }

    fn push_str(&mut self, s: &str) -> Result<(), Error> {
        let s = s.to_ascii_lowercase();
        if let Some(def) = self.definitions.get(&s) {
            return self.eval(def.clone().as_str());
        }
        let stack_item = match s.as_str() {
            s if let Ok(s) = s.parse::<Value>() => StackItem::Number(s),
            "+" => StackItem::Operation(Operation::Plus),
            "-" => StackItem::Operation(Operation::Minus),
            "*" => StackItem::Operation(Operation::Mul),
            "/" => StackItem::Operation(Operation::Div),
            "dup" => StackItem::Manipulation(Manipulation::Dup),
            "drop" => StackItem::Manipulation(Manipulation::Drop),
            "swap" => StackItem::Manipulation(Manipulation::Swap),
            "over" => StackItem::Manipulation(Manipulation::Over),
            _ => return Err(Error::UnknownWord),
        };
        self.push(stack_item)?;
        Ok(())
    }

    fn push(&mut self, stack_item: StackItem) -> Result<(), Error> {
        match stack_item {
            StackItem::Number(n) => {
                self.stack.push(n);
            }
            StackItem::Operation(op) => {
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
            StackItem::Manipulation(manip) => match manip {
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
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        if input.starts_with(":") {
            return self.definition(input);
        }
        input.split(' ').try_for_each(|s| self.push_str(s))
    }
}
