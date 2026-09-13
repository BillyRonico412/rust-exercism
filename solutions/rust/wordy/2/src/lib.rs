#[derive(PartialEq)]
enum Token {
    What,
    Is,
    Number(i32),
    Plus,
    Minus,
    Multiplied,
    Dividied,
    By,
    Unknwon,
}

fn scan(s: &str) -> Token {
    match s {
        "What" => Token::What,
        "is" => Token::Is,

        s if let Ok(number) = s.parse::<i32>() => Token::Number(number),

        "plus" => Token::Plus,
        "minus" => Token::Minus,
        "multiplied" => Token::Multiplied,
        "divided" => Token::Dividied,

        "by" => Token::By,

        _ => Token::Unknwon,
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(PartialEq)]
enum State {
    Init,
    What,
    Is,
    Number,
    Op,
    By,
}

impl Default for State {
    fn default() -> Self {
        State::Init
    }
}

#[derive(Default)]
struct StateMachine {
    state: State,
    op: Option<Op>,
    current_number: Option<i32>,
}

impl StateMachine {
    pub fn next(&mut self, token: Token) -> bool {
        match self.state {
            State::Init => match token {
                Token::What => {
                    self.state = State::What;
                    true
                }
                _ => false,
            },
            State::What => match token {
                Token::Is => {
                    self.state = State::Is;
                    true
                }
                _ => false,
            },
            State::Is => match token {
                Token::Number(n) => {
                    self.current_number = Some(n);
                    self.state = State::Number;
                    true
                }
                _ => false,
            },
            State::Number => match token {
                Token::Plus => {
                    self.op = Some(Op::Plus);
                    self.state = State::Op;
                    true
                }
                Token::Minus => {
                    self.op = Some(Op::Minus);
                    self.state = State::Op;
                    true
                }
                Token::Multiplied => {
                    self.op = Some(Op::Mul);
                    self.state = State::By;
                    true
                }
                Token::Dividied => {
                    self.op = Some(Op::Div);
                    self.state = State::By;
                    true
                }
                _ => false,
            },
            State::By => match token {
                Token::By => {
                    self.state = State::Op;
                    true
                }
                _ => false,
            },
            State::Op => match token {
                Token::Number(n) => {
                    let Some(current_number) = self.current_number.clone() else {
                        return false;
                    };
                    let Some(op) = self.op.clone() else {
                        return false;
                    };

                    self.current_number = Some(match op {
                        Op::Plus => current_number + n,
                        Op::Minus => current_number - n,
                        Op::Mul => current_number * n,
                        Op::Div => current_number / n,
                    });

                    self.op = None;
                    self.state = State::Number;
                    true
                }
                _ => false,
            },
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let mut command = command.to_string();
    command.pop();

    let mut state_machine = StateMachine::default();
    for token in command.split(" ").map(scan) {
        if token == Token::Unknwon || !state_machine.next(token) {
            return None;
        }
    }
    if state_machine.state != State::Number {
        return None;
    }
    state_machine.current_number
}
