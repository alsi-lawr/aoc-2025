use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct ProductInfo {
    pub lower_id: u64,
    pub upper_id: u64,
}

#[derive(Debug)]
pub enum ProductParsingError {
    EmptyFile { raw: String },
    EmptyProduct { raw: String },
    InvalidRange { raw: String },
}

impl fmt::Display for ProductParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductParsingError::EmptyFile { raw } => {
                write!(f, "Empty file (raw: {:?})", raw)
            }
            ProductParsingError::EmptyProduct { raw } => {
                write!(f, "Empty product (raw: {:?})", raw)
            }
            ProductParsingError::InvalidRange { raw } => {
                write!(f, "Empty product (raw: {:?})", raw)
            }
        }
    }
}

impl Error for ProductParsingError {}

impl FromStr for ProductInfo {
    type Err = ProductParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.to_owned();
        let s = s.trim();
        if s.is_empty() {
            return Err(ProductParsingError::EmptyProduct { raw });
        }
        let (lower_raw, upper_raw) = s
            .split_once('-')
            .ok_or(ProductParsingError::InvalidRange { raw: raw.clone() })?;
        let lower: u64 =
            lower_raw
                .parse::<u64>()
                .map_err(|_| ProductParsingError::InvalidRange {
                    raw: lower_raw.to_string(),
                })?;
        let upper: u64 =
            upper_raw
                .parse::<u64>()
                .map_err(|_| ProductParsingError::InvalidRange {
                    raw: upper_raw.to_string(),
                })?;
        Ok(Self {
            lower_id: lower,
            upper_id: upper,
        })
    }
}
