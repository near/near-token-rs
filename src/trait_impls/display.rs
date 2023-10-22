use crate::{NearToken, NearTokenError, ONE_MILLINEAR};

/// NearToken Display implementation rounds up the token amount to the relevant precision point.
/// There are 4 breakpoints:
/// 1. exactly 0 NEAR
/// 2. <0.001 NEAR
/// 3. 0.001 - 0.999 NEAR (uses 3 digits after the floating point)
/// 4. >1 NEAR (uses 2 digits after the floating point)
impl std::fmt::Display for NearToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == NearToken::from_yoctonear(0) {
            write!(f, "0 NEAR")
        } else if *self < NearToken::from_millinear(1) {
            write!(f, "<0.001 NEAR")
        } else if *self <= NearToken::from_millinear(999) {
            let millinear_rounded_up =
                self.as_yoctonear().saturating_add(ONE_MILLINEAR - 1) / ONE_MILLINEAR;
            write!(f, "0.{:03} NEAR", millinear_rounded_up)
        } else {
            let near_rounded_up =
                self.as_yoctonear().saturating_add(10 * ONE_MILLINEAR - 1) / ONE_MILLINEAR / 10;
            write!(
                f,
                "{}.{:02} NEAR",
                near_rounded_up / 100,
                near_rounded_up % 100
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
        for (near_tokens, expected_display) in [
            (NearToken::from_yoctonear(0), "0 NEAR"),
            (NearToken::from_yoctonear(1), "<0.001 NEAR"),
            (NearToken::from_yoctonear(10u128.pow(21) - 1), "<0.001 NEAR"),
            (NearToken::from_yoctonear(10u128.pow(21)), "0.001 NEAR"),
            (NearToken::from_yoctonear(10u128.pow(21) + 1), "0.002 NEAR"),
            (NearToken::from_yoctonear(10u128.pow(21) * 2), "0.002 NEAR"),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 200),
                "0.200 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 999),
                "0.999 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 999 + 1),
                "1.00 NEAR",
            ),
            (NearToken::from_yoctonear(10u128.pow(24) - 1), "1.00 NEAR"),
            (NearToken::from_yoctonear(10u128.pow(24)), "1.00 NEAR"),
            (NearToken::from_yoctonear(10u128.pow(24) + 1), "1.01 NEAR"),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 1234),
                "1.24 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 1500),
                "1.50 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 10000),
                "10.00 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 10500),
                "10.50 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 100000 - 1),
                "100.00 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 100000),
                "100.00 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 100500),
                "100.50 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 100000000),
                "100000.00 NEAR",
            ),
            (
                NearToken::from_yoctonear(10u128.pow(21) * 100000500),
                "100000.50 NEAR",
            ),
        ] {
            assert_eq!(
                near_tokens.to_string(),
                expected_display,
                "tokens: {}",
                near_tokens.as_yoctonear()
            );
        }
    }
}
