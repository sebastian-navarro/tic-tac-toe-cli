#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub symbol: char,
}

impl Player {
    pub fn new(name:String, symbol: char) -> Self {
        return Self {
            name,
            symbol,
        }
    }
}