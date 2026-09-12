pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut vec = vec![];
    let mut i = 0;
    loop {
        if i == rna.len() {
            break;
        }
        let Some(codon) = rna.get(i..(i + 3)) else {
            return None;
        };
        let amino_acid = match codon {
            "AUG" => Some("Methionine"),
            "UUU" | "UUC" => Some("Phenylalanine"),
            "UUA" | "UUG" => Some("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
            "UAU" | "UAC" => Some("Tyrosine"),
            "UGU" | "UGC" => Some("Cysteine"),
            "UGG" => Some("Tryptophan"),
            "UAA" | "UAG" | "UGA" => break,
            _ => None,
        };
        let Some(amino_acid) = amino_acid else {
            return None;
        };
        vec.push(amino_acid);
        i += 3;
    }
    Some(vec)
}
