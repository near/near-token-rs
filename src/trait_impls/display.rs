use crate::{NearToken, NearTokenError, ONE_MILI_NEAR};

/// NearGas Display implementation rounds up the gas usage to the relevant precision point.
/// There are 4 breakpoints:
/// 1. exactly 0 Tgas
/// 2. <0.001 Tgas
/// 3. 0.001 - 0.999 Tgas (uses 3 digits after the floating point)
/// 4. >1 Tgas (uses 1 digit after the floating point)
impl std::fmt::Display for NearToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == NearToken::from_yoctonear(0) {
            write!(f, "0 Near")
        } else if *self < NearToken::from_mili_near(1) {
            write!(f, "<0.001 Near")
        } else if *self <= NearToken::from_mili_near(999) {
            let gigagas_rounded_up = self.as_yoctonear().saturating_add(ONE_MILI_NEAR - 1) / ONE_MILI_NEAR;
            write!(f, "0.{:03} Near", gigagas_rounded_up)
        } else {
            let terragas_rounded_up =
                self.as_yoctonear().saturating_add(100 * ONE_MILI_NEAR - 1) / ONE_MILI_NEAR / 100;
            write!(
                f,
                "{}.{} Near",
                terragas_rounded_up / 10,
                terragas_rounded_up % 10
            )
        }
    }
}

impl std::fmt::Display for NearTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NearTokenError::IncorrectNumber(err) => write!(f, "Incorrect number: {:?}", err),
            NearTokenError::IncorrectUnit(err) => write!(f, "Incorrect unit: {}", err),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::NearToken;

    #[test]
    fn test_display() {
        for (near_gas, expected_display) in [
            (NearToken::from_yoctonear(0), "0 Near"),
            (NearToken::from_yoctonear(1), "<0.001 Near"),
            (NearToken::from_yoctonear(999_999_999), "<0.001 Near"),
            (NearToken::from_yoctonear(10u128.pow(21)), "0.001 Near"),
            (NearToken::from_yoctonear(10u128.pow(21) + 1), "0.002 Near"),
            (NearToken::from_yoctonear(10u128.pow(21) * 2), "0.002 Near"),
            //(NearToken::from_yoctonear(200_000_000_000), "0.200 Near"),
            //(NearToken::from_yoctonear(999_000_000_000), "0.999 Near"),
            //(NearToken::from_yoctonear(999_000_000_001), "1.0 Near"),
            //(NearToken::from_yoctonear(999_999_999_999), "1.0 Near"),
            //(NearToken::from_yoctonear(1_000_000_000_000), "1.0 Near"),
            //(NearToken::from_yoctonear(1_000_000_000_001), "1.1 Near"),
            //(NearToken::from_yoctonear(1_234_567_000_000), "1.3 Near"),
            //(NearToken::from_yoctonear(1_500_000_000_000), "1.5 Near"),
            //(NearToken::from_yoctonear(10_000_000_000_000), "10.0 Near"),
            //(NearToken::from_yoctonear(10_500_000_000_000), "10.5 Near"),
            //(NearToken::from_yoctonear(99_999_999_999_999), "100.0 Near"),
            //(NearToken::from_yoctonear(100_000_000_000_000), "100.0 Near"),
            //(NearToken::from_yoctonear(100_500_000_000_000), "100.5 Near"),
            //(NearToken::from_yoctonear(1_000_500_000_000_000), "1000.5 Near"),
            //(
            //    NearToken::from_yoctonear(1_000_000_500_000_000_000),
            //    "1000000.5 Near",
            //),
        ] {
            assert_eq!(
                near_gas.to_string(),
                expected_display,
                "gas: {}",
                near_gas.as_yoctonear()
            );
        }
    }
}
