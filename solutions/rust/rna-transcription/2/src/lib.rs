#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: String,
}

const DNA_SEQ: &str = "ACGT";

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !DNA_SEQ.contains(c) {
                return Err(i);
            }
        }
        Ok(Dna {
            seq: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            seq: self
                .seq
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

const RNA_SEQ: &str = "ACGU";
impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !RNA_SEQ.contains(c) {
                return Err(i);
            }
        }
        Ok(Rna {
            seq: rna.to_string(),
        })
    }
}
