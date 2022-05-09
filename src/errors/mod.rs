use core::fmt;

#[derive(Debug, Clone)]
pub enum Errors {
    ColorAlreadyUsed,
    ColorOutOfRange
}

#[derive(Debug, Clone)]
pub struct ColorAlreadyUsed;

impl fmt::Display for ColorAlreadyUsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You cannot use this color. This color is already used")
    }
}

#[derive(Debug, Clone)]
pub struct ColorOutOfRange;

impl fmt::Display for ColorOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You cannot use this color. This color more than max color index")
    }
}