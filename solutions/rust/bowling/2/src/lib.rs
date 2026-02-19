#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Frame {
    OpenFrame(u16, u16),
    Spare(u16),
    Strike,
    TenthFrame(u16, u16, u16),
    None,
}

pub struct BowlingGame {
    frames: [Frame; 10],
    frame_rolls: (Option<u16>, Option<u16>),
    turn: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: [Frame::None; 10],
            frame_rolls: (None, None),
            turn: 0,
        }
    }

    fn update_frame(&mut self, frame: Frame) {
        self.frames[self.turn] = frame;
        self.turn += 1;
        self.frame_rolls = (None, None)
    }

    fn handle_basic_game(&mut self, pins: u16) -> Result<(), Error> {
        match self.frame_rolls {
            (None, None) if pins == 10 => {
                self.update_frame(Frame::Strike);
                Ok(())
            }
            (None, None) => {
                self.frame_rolls.0 = Some(pins);
                Ok(())
            }
            (Some(x), None) => {
                if x + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else if x + pins == 10 {
                    self.update_frame(Frame::Spare(x));
                    Ok(())
                } else {
                    self.update_frame(Frame::OpenFrame(x, pins));
                    Ok(())
                }
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    fn handle_tenth_game(&mut self, pins: u16) -> Result<(), Error> {
        match self.frame_rolls {
            (None, None) => {
                self.frame_rolls.0 = Some(pins);
                Ok(())
            }
            (Some(x), None) => {
                if x < 10 && x + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else if x != 10 && x + pins != 10 {
                    self.update_frame(Frame::TenthFrame(x, pins, 0));
                    Ok(())
                } else {
                    self.frame_rolls.1 = Some(pins);
                    Ok(())
                }
            }
            (Some(x), Some(y)) => {
                if x == 10 && y != 10 && y + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else {
                    self.update_frame(Frame::TenthFrame(x, y, pins));
                    Ok(())
                }
            }
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.turn < 9 {
            self.handle_basic_game(pins)
        } else if self.turn == 9 {
            self.handle_tenth_game(pins)
        } else {
            Err(Error::GameComplete)
        }
    }

    fn get_next_pins(&self, turn: usize) -> (u16, u16) {
        let next_frame = self.frames[turn + 1];
        match next_frame {
            Frame::None => panic!(),
            Frame::OpenFrame(x, y) => (x, y),
            Frame::TenthFrame(x, y, _) => (x, y),
            Frame::Spare(x) => (x, 10 - x),
            Frame::Strike => (10, self.get_next_pins(turn + 1).0),
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.iter().any(|&frame| frame == Frame::None) {
            return None;
        }
        let mut score = 0u16;
        for (i, &frame) in self.frames.iter().enumerate() {
            score += match frame {
                Frame::None => panic!(),
                Frame::OpenFrame(x, y) => x + y,
                Frame::TenthFrame(x, y, z) => x + y + z,
                Frame::Spare(_) => 10 + self.get_next_pins(i).0,
                Frame::Strike => {
                    let (x, y) = self.get_next_pins(i);
                    10 + x + y
                }
            }
        }
        Some(score)
    }
}
