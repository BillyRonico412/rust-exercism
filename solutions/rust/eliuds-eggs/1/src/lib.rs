pub fn egg_count(display_value: u32) -> usize {
    let mut pow_2 = 1;
    let mut nb = 0;
    while pow_2 <= display_value {
        if display_value & pow_2 == 1 {
            nb += 1
        }
        pow_2 *= 2
    }
    nb
}
