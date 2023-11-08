use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug)]
pub struct TruthTable {
    pub header: Vec<String>,
    pub lines: Vec<Vec<bool>>,
}

impl Display for TruthTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        self.header
            .iter()
            .try_for_each(|var| write!(f, "{var} | "))?;
        writeln!(f)?;
        self.header
            .iter()
            .try_for_each(|var| write!(f, "{}", "-".repeat(var.len() + 3)))?;
        writeln!(f)?;
        self.lines.iter().try_for_each(|line| {
            debug_assert_eq!(line.len(), self.header.len());
            self.header.iter().enumerate().try_for_each(|(i, var)| {
                let b = line[i];
                let t = if b { "T" } else { "F" };
                write!(f, "{t:w$} | ", w = var.len())
            })?;
            writeln!(f)
        })
    }
}
