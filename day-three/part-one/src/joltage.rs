use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct JoltageLayout<const N: usize> {
    pub batteries: [u8; N],
}

#[derive(Debug)]
pub enum JoltageParsingError {
    EmptyJoltage { raw: String },
}

impl fmt::Display for JoltageParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JoltageParsingError::EmptyJoltage { raw } => {
                write!(f, "Empty joltage layout (raw: {:?})", raw)
            }
        }
    }
}

impl Error for JoltageParsingError {}

impl<const N: usize> FromStr for JoltageLayout<N> {
    type Err = JoltageParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.to_owned();
        let raw = raw.trim();
        if raw.is_empty() {
            return Err(JoltageParsingError::EmptyJoltage {
                raw: raw.to_string(),
            });
        }

        let batteries_slice: Vec<u8> = raw.as_bytes().iter().map(|b| b - b'0').collect();
        let mut batteries = [0u8; N];
        batteries.copy_from_slice(batteries_slice.as_slice());

        Ok(Self { batteries })
    }
}
