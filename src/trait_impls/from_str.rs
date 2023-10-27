use crate::{NearToken, NearTokenError, ONE_NEAR};

impl std::str::FromStr for NearToken {
    type Err = NearTokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uppercase_s = s.trim().to_ascii_uppercase();
        let (value, unit) = uppercase_s.split_at(
            s.find(|c: char| c.is_ascii_alphabetic())
                .ok_or_else(|| NearTokenError::IncorrectUnit(s.to_owned()))?,
        );
        let unit_precision = match unit {
            "YN" | "YNEAR" | "YOCTONEAR" => 1,
            "NEAR" | "N" => ONE_NEAR,
            _ => return Err(NearTokenError::IncorrectUnit(s.to_owned())),
        };
        Ok(NearToken::from_yoctonear(
            crate::utils::parse_decimal_number(value.trim(), unit_precision)
                .map_err(NearTokenError::IncorrectNumber)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{DecimalNumberParsingError, NearToken, NearTokenError};

    #[test]
    fn parse_decimal_number() {
        let data = "0.123456 near";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas.unwrap(),
            NearToken::from_yoctonear(123456000000000000000000)
        );
    }
    #[test]
    fn parse_number_with_decimal_part() {
        let data = "11.123456 near";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas.unwrap(),
            NearToken::from_yoctonear(11123456000000000000000000)
        );
    }

    #[test]
    fn parse_yocto_number() {
        let data = "123456 YN";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(gas.unwrap(), NearToken::from_yoctonear(123456));
    }

    #[test]
    fn doubledot() {
        let data = "1.1.1 Near";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearTokenError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("1.1.1".to_owned())
            ))
        )
    }

    #[test]
    fn space_after_dot() {
        let data = "1. 0 near";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearTokenError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("1. 0".to_owned())
            ))
        )
    }

    #[test]
    fn incorect_currency() {
        let data = "0 pas";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(gas, Err(NearTokenError::IncorrectUnit(data.to_owned())))
    }

    #[test]
    fn without_currency() {
        let data = "0";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(gas, Err(NearTokenError::IncorrectUnit("0".to_owned())))
    }

    #[test]
    fn invalid_whole() {
        let data = "-1 Near";
        let gas: Result<NearToken, NearTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearTokenError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("-1".to_owned())
            ))
        )
    }

    #[test]
    fn test_from_str_f64_gas_without_int() {
        let near_gas = NearToken::from_str(".055 ynear").unwrap_err();
        assert_eq!(
            near_gas,
            NearTokenError::IncorrectNumber(DecimalNumberParsingError::InvalidNumber(
                ".055".to_string()
            ))
        );
    }

    #[test]
    fn test_from_str_without_unit() {
        let near_gas = NearToken::from_str("100").unwrap_err();
        assert_eq!(near_gas, NearTokenError::IncorrectUnit("100".to_string()));
    }

    #[test]
    fn test_from_str_incorrect_unit() {
        let near_gas = NearToken::from_str("100 UAH").unwrap_err();
        assert_eq!(
            near_gas,
            NearTokenError::IncorrectUnit("100 UAH".to_string())
        );
    }

    #[test]
    fn test_from_str_invalid_double_dot() {
        let near_gas = NearToken::from_str("100.55.").unwrap_err();
        assert_eq!(
            near_gas,
            NearTokenError::IncorrectUnit("100.55.".to_string())
        );
    }

    #[test]
    fn test_from_str_large_fractional_part() {
        let near_gas = NearToken::from_str("100.1111122222333 ynear").unwrap_err(); // 13 digits after "."
        assert_eq!(
            near_gas,
            NearTokenError::IncorrectNumber(DecimalNumberParsingError::LongFractional(
                "1111122222333".to_string()
            ))
        );
    }
}
