#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NearTokenError {
    InvalidTokensAmount(crate::utils::DecimalNumberParsingError),
    InvalidTokenUnit(String),
}

impl std::fmt::Display for NearTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NearTokenError::InvalidTokensAmount(err) => write!(f, "invalid tokens amount: {}", err),
            NearTokenError::InvalidTokenUnit(unit) => write!(f, "invalid token unit: {}", unit),
        }
    }
}

impl std::error::Error for NearTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NearTokenError::InvalidTokensAmount(err) => Some(err),
            NearTokenError::InvalidTokenUnit(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_near_token_error_display() {
        assert_eq!(
            format!(
                "{}",
                NearTokenError::InvalidTokensAmount(
                    crate::utils::DecimalNumberParsingError::InvalidNumber("abc".to_owned())
                )
            ),
            "invalid tokens amount: invalid number: abc"
        );
        assert_eq!(
            format!(
                "{}",
                NearTokenError::InvalidTokensAmount(
                    crate::utils::DecimalNumberParsingError::LongWhole("999999999999.0".to_owned())
                )
            ),
            "invalid tokens amount: too long whole part: 999999999999.0"
        );
        assert_eq!(
            format!(
                "{}",
                NearTokenError::InvalidTokensAmount(
                    crate::utils::DecimalNumberParsingError::LongFractional(
                        "0.999999999999".to_owned()
                    )
                )
            ),
            "invalid tokens amount: too long fractional part: 0.999999999999"
        );
        assert_eq!(
            format!("{}", NearTokenError::InvalidTokenUnit("abc".to_owned())),
            "invalid token unit: abc"
        );
    }
}
