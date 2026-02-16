pub fn egg_count(display_value: u32) -> usize {
    let mut nb = 0;
    let mut pow_2 = 1;
    while pow_2 <= display_value {
        if display_value & pow_2 != 0 {
            nb += 1
        }
        pow_2 *= 2
    }
    nb
}
