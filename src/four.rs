
#[derive(Debug,Clone)]
pub struct Pizza {
    toppings: Vec<String>
}

impl Pizza {
    pub fn new(value:Vec<String>) -> Self {
        Self { toppings: value }
    }

    pub fn toppings(&self) -> &[String] {
        self.toppings.as_ref()
    }

    pub fn toppings_mut(&mut self) -> &mut Vec<String> {
        &mut self.toppings
    }

    pub fn set_toppings(&mut self, toppings:Vec<String>) {
        self.toppings = toppings;
    }
}

use std::path::Path;


#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadLinneArgument(usize),
    
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
        
    }
}


pub fn read_nth_line(path: &Path, n: usize) -> Result<String,Error> {
    
    if n < 1 {
        return Err(Error::BadLinneArgument(0));
    }
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;

    let mut reader_lines = BufReader::new(file).lines();

    reader_lines.nth(n-1)
                .map(|result| result.map_err(|err| err.into()))
                .unwrap_or_else(|| Err(Error::BadLinneArgument(n)))
}


