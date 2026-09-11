#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut vec = vec![];
    for value in values.iter().rev() {
        let mut current_byte = 0u8;

        let mut offset_in_value = 0;
        let mut offset_in_byte = 0;

        let size = (0..32)
            .rev()
            .find(|&ptr| (value & (1 << ptr)) != 0)
            .unwrap_or_default();

        loop {
            if offset_in_byte == 7 {
                if offset_in_value != 7 {
                    current_byte |= 1 << 7;
                }
                vec.push(current_byte);
                current_byte = 0;
                offset_in_byte = 0;
            }

            let bit = value & (1 << offset_in_value);
            let bit = (bit != 0) as u8;
            current_byte |= bit << offset_in_byte;
            offset_in_value += 1;
            offset_in_byte += 1;

            if offset_in_value == size + 1 {
                if offset_in_value > 7 {
                    current_byte |= 1 << 7;
                }
                vec.push(current_byte);
                break;
            }
        }
    }

    vec.reverse();
    vec
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut vec: Vec<u32> = vec![];
    let mut current_value = 0u32;
    if let Some(last) = bytes.last() {
        let first_bit = last & (1 << 7);
        let first_bit = (first_bit != 0) as u8;
        if first_bit != 0 {
            return Err(Error::IncompleteNumber);
        }
    }
    for byte in bytes {
        let first_bit = byte & (1 << 7);
        let first_bit = (first_bit != 0) as u8;
        let byte = byte & !(1 << 7);
        current_value = (current_value << 7) | (byte as u32);
        if first_bit == 0 {
            vec.push(current_value);
            current_value = 0;
        }
    }
    Ok(vec)
}
