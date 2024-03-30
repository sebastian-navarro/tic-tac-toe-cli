use core::fmt;



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

    pub fn insert_new_symbol(&mut self, row: usize , col: usize, symbol: char) -> bool {
        if row >= self.size || col >= self.size {
            println!("row or col should be less than {}", self.size);
            return false
        }
        if self.cells[row as usize][col as usize] != '-' {
            println!("row and col must be empty");
            return false
        }

        self.cells[row as usize][col as usize] = symbol;
        return true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                fmt.write_str(&cell.to_string())?;
                fmt.write_str(" ")?;
            }
            fmt.write_str("\n")?;
        }
        Ok(())
    }
}