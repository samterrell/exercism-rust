use std::result;

pub struct PascalsTriangle(usize);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count as usize)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.0 == 0 {
            return vec![];
        }
        let mut result = Vec::with_capacity(self.0);
        result.push(vec![1]);
        for r in 1..self.0 {
            let mut row = Vec::with_capacity(r as usize);
            row.push(1);
            for w in result.last().unwrap().windows(2) {
                row.push(w[0] + w[1]);
            }
            row.push(1);
            result.push(row);
        }
        result
    }
}
