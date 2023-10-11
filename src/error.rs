#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NearTokenError {
    IncorrectNumber(crate::utils::DecimalNumberParsingError),
    IncorrectUnit(String),
}


