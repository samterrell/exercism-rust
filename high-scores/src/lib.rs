#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        Some(*self.0.last()?)
    }

    pub fn personal_best(&self) -> Option<u32> {
        Some(*self.0.iter().max()?)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut max = [None; 3];
        for score in self.0 {
            match max {
                [None, _, _] => max[0] = Some(*score),
                [Some(n), _, _] if *score > n => max = [Some(*score), max[0], max[1]],
                [_, None, _] => max[1] = Some(*score),
                [_, Some(n), _] if *score > n => max = [max[0], Some(*score), max[1]],
                [_, _, None] => max[2] = Some(*score),
                [_, _, Some(n)] if *score > n => max = [max[0], max[1], Some(*score)],
                _ => (),
            }
        }
        max.iter().flatten().map(|v| *v).collect()
    }
}
