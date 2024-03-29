

#[derive(Debug)]
pub struct Board{
    pub size: usize,
    pub cells: Vec<Vec<char>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let cells = vec![vec!['-'; size as usize]; size as usize];
        return Self { size, cells }
    }
}