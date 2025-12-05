use std::io;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct ProductInfo {
    pub lower_id: u64,
    pub upper_id: u64,
}

#[derive(Debug, Error)]
pub enum ProductParsingError {
    #[error("Failed to parse file (raw: {raw})")]
    IoError { raw: String },

    #[error("Empty file (raw: {raw})")]
    EmptyFile { raw: String },

    #[error("Empty product (raw: {raw})")]
    EmptyProduct { raw: String },

    #[error("Invalid range (raw: {raw})")]
    InvalidRange { raw: String },
}

impl From<io::Error> for ProductParsingError {
    fn from(e: io::Error) -> Self {
        ProductParsingError::IoError { raw: e.to_string() }
    }
}

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
