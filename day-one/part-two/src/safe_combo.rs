use std::error::Error;
use std::fmt;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboDirection {
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafeCombo {
    pub direction: ComboDirection,
    pub steps: i16,
}

pub struct PasswordCounter {
    pub total: i16,
    pub password: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafeComboParsingError {
    Empty { raw: String },
    InvalidDirection { raw: String },
    InvalidRotation { raw: String },
}

impl fmt::Display for SafeComboParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafeComboParsingError::Empty { raw } => {
                write!(f, "empty combo string (raw: {:?})", raw)
            }
            SafeComboParsingError::InvalidDirection { raw } => {
                write!(f, "invalid rotation direction (raw: {:?})", raw)
            }
            SafeComboParsingError::InvalidRotation { raw } => {
                write!(f, "invalid rotation steps (raw: {:?})", raw)
            }
        }
    }
}

impl Error for SafeComboParsingError {}

impl FromStr for SafeCombo {
    type Err = SafeComboParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.to_owned();
        let s = s.trim();
        if s.is_empty() {
            return Err(SafeComboParsingError::Empty { raw });
        }

        let (direction_raw, steps_raw) = s.split_at(1);
        let direction = match direction_raw {
            "L" | "l" => ComboDirection::L,
            "R" | "r" => ComboDirection::R,
            _ => return Err(SafeComboParsingError::InvalidDirection { raw }),
        };

        let steps: i16 = steps_raw
            .parse()
            .map_err(|_| SafeComboParsingError::InvalidRotation {
                raw: steps_raw.to_owned(),
            })?;

        Ok(Self { direction, steps })
    }
}

impl ops::Add<&SafeCombo> for PasswordCounter {
    type Output = PasswordCounter;

    fn add(self, _rhs: &SafeCombo) -> Self::Output {
        let rem = _rhs.steps % 100;
        let full_turns = _rhs.steps / 100;
        match _rhs.direction {
            ComboDirection::L => PasswordCounter {
                total: (self.total - _rhs.steps).rem_euclid(100),
                password: self.password
                    + full_turns
                    + i16::from(rem >= self.total && self.total != 0),
            },
            ComboDirection::R => PasswordCounter {
                total: (self.total + _rhs.steps).rem_euclid(100),
                password: self.password
                    + full_turns
                    + i16::from(rem > 0 && (self.total + rem) >= 100),
            },
        }
    }
}
