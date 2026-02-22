/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut pangram_array_checker = [false; 26];
    sentence.bytes().for_each(|b| {
        if !b.is_ascii_alphabetic() {
            return;
        }
        let b_lowercase = b.to_ascii_lowercase();
        pangram_array_checker[(b_lowercase - b'a') as usize] = true;
    });
    pangram_array_checker.iter().all(|&b| b)
}
