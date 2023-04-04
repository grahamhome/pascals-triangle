mod tests;

pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = PascalsTriangle(vec![]);
        if row_count > 0 {
            triangle.0.push(vec![1]);
            (0..row_count - 1).for_each(|_| triangle.0.push(triangle.next_row()));
        }
        triangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }

    fn next_row(&self) -> Vec<u32> {
        let prev_row = self.0.last().unwrap();
        vec![prev_row.first().unwrap().to_owned()]
            .into_iter()
            .chain(prev_row.windows(2).map(|w| w.into_iter().sum()))
            .chain(vec![prev_row.last().unwrap().to_owned()])
            .collect()
    }
}
