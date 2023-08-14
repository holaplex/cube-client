use std::fmt;

pub enum TimeGranularity {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl fmt::Display for TimeGranularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::Year => write!(f, "year"),
        }
    }
}
