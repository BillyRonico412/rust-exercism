#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_score = self.scores().to_vec();
        sorted_score.sort();
        sorted_score.iter().rev().take(3).copied().collect()
    }
}
