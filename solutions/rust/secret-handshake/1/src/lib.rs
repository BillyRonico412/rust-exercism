enum Handshake {
    Wink,
    DoubleBlink,
    CloseEyes,
    Jump,
    Reverse,
}

impl Handshake {
    fn action(&self) -> &str {
        match self {
            Handshake::Wink => "wink",
            Handshake::DoubleBlink => "double blink",
            Handshake::CloseEyes => "close your eyes",
            Handshake::Jump => "jump",
            Handshake::Reverse => "",
        }
    }
    fn bit(&self) -> u8 {
        match self {
            Handshake::Wink => 0b00001,
            Handshake::DoubleBlink => 0b00010,
            Handshake::CloseEyes => 0b00100,
            Handshake::Jump => 0b01000,
            Handshake::Reverse => 0b10000,
        }
    }
}

pub fn actions(n: u8) -> Vec<&'static str> {
    let base_handshake = [
        Handshake::Wink,
        Handshake::DoubleBlink,
        Handshake::CloseEyes,
        Handshake::Jump,
    ]
    .iter()
    .filter_map(|handshake| ((n & handshake.bit()) != 0).then_some(handshake.action()));
    if (n & Handshake::Reverse.bit()) == 0 {
        base_handshake.collect()
    } else {
        base_handshake.rev().collect()
    }
}
