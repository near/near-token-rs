//! A `NearToken` type to represent a value of Near.
//!
//! Each `Neartokens` is composed of a floating point number of tokens where each integer unit is equal to one yocto-Near.
//! `NearToken` is implementing the common trait `FromStr`. Also, have utils function to parse from `str` into `u128`.
//!
//! # Examples
//! ```
//! use near_token::NearToken;
//!
//! let one_near = NearToken::from_yoctonear(10_u128.pow(24));
//! assert_eq!(one_near, NearToken::from_near(1));
//! assert_eq!(one_near, NearToken::from_millinear(1000));
//! ```
//!
//! # Crate features
//!
//! * **borsh** (optional) -
//!   When enabled allows `NearToken` to serialized and deserialized by `borsh`.
//!
//! * **serde** (optional) -
//!   When enabled allows `NearToken` to serialized and deserialized by `serde`.
//!
//! * **schemars** (optional) -
//!   Implements `schemars::JsonSchema` for `NearToken`.
//!
//! * **interactive-clap** (optional) -
//!   Implements `interactive_clap::ToCli` for `NearToken`.
mod error;

mod utils;

mod trait_impls;

pub use self::error::NearTokenError;
pub use self::utils::DecimalNumberParsingError;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize)
)]
#[cfg_attr(feature = "abi", derive(borsh::BorshSchema))]
#[repr(transparent)]
pub struct NearToken {
    inner: u128,
}

const ONE_NEAR: u128 = 10_u128.pow(24);
const ONE_MILLINEAR: u128 = 10_u128.pow(21);

impl NearToken {
    /// Maximum value for NearToken (u128::MAX)
    pub const MAX: NearToken = NearToken::from_yoctonear(u128::MAX);
    /// Zero value for NearToken
    pub const ZERO: NearToken = NearToken::from_yoctonear(0);
    
    /// `from_yoctonear` is a function that takes value by a number of yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!( NearToken::from_yoctonear(10u128.pow(21)), NearToken::from_millinear(1))
    /// ```
    pub const fn from_yoctonear(inner: u128) -> Self {
        Self { inner }
    }

    /// `from_millinear` is a function that takes value by a number of mili-near and converts it to an equivalent to the yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_millinear(1), NearToken::from_yoctonear(10u128.pow(21)))
    /// ```
    pub const fn from_millinear(inner: u128) -> Self {
        Self {
            inner: inner * ONE_MILLINEAR,
        }
    }

    /// `from_near` is a function that takes value by a number of near and converts it to an equivalent to the yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_near(1), NearToken::from_yoctonear(10u128.pow(24)))
    /// ```
    pub const fn from_near(inner: u128) -> Self {
        Self {
            inner: inner * ONE_NEAR,
        }
    }

    /// `as_near` is a function that converts number of yocto-near to an equivalent to the near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10u128.pow(24)).as_near(), 1)
    /// ```
    pub const fn as_near(&self) -> u128 {
        self.inner / ONE_NEAR
    }

    /// `as_millinear` is a function that converts number of yocto-near to an equivalent to the mili-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10u128.pow(21)).as_millinear(), 1)
    /// ```
    pub const fn as_millinear(&self) -> u128 {
        self.inner / ONE_MILLINEAR
    }

    /// `as_yoctonear` is a function that shows a number of yocto-near.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10).as_yoctonear(), 10)
    /// ```
    pub const fn as_yoctonear(&self) -> u128 {
        self.inner
    }

    /// `is_zero` is a boolian function that checks `NearToken`
    /// if a `NearToken` inner is zero, returns true.
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(0).is_zero(), true)
    /// ```
    pub const fn is_zero(&self) -> bool {
        self.inner == 0
    }

    /// Checked integer addition. Computes self + rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// use std::u128;
    /// assert_eq!(NearToken::from_yoctonear(u128::MAX -2).checked_add(NearToken::from_yoctonear(2)), Some(NearToken::from_yoctonear(u128::MAX)));
    /// assert_eq!(NearToken::from_yoctonear(u128::MAX -2).checked_add(NearToken::from_yoctonear(3)), None);
    /// ```
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        if let Some(near) = self.as_yoctonear().checked_add(rhs.as_yoctonear()) {
            Some(Self::from_yoctonear(near))
        } else {
            None
        }
    }

    /// Checked integer subtraction. Computes self - rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(2).checked_sub(NearToken::from_yoctonear(2)), Some(NearToken::from_yoctonear(0)));
    /// assert_eq!(NearToken::from_yoctonear(2).checked_sub(NearToken::from_yoctonear(3)), None);
    /// ```
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        if let Some(near) = self.as_yoctonear().checked_sub(rhs.as_yoctonear()) {
            Some(Self::from_yoctonear(near))
        } else {
            None
        }
    }

    /// Checked integer multiplication. Computes self * rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// use std::u128;
    /// assert_eq!(NearToken::from_yoctonear(2).checked_mul(2), Some(NearToken::from_yoctonear(4)));
    /// assert_eq!(NearToken::from_yoctonear(u128::MAX).checked_mul(2), None)
    pub const fn checked_mul(self, rhs: u128) -> Option<Self> {
        if let Some(near) = self.as_yoctonear().checked_mul(rhs) {
            Some(Self::from_yoctonear(near))
        } else {
            None
        }
    }

    /// Checked integer division. Computes self / rhs, returning None if rhs == 0.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10).checked_div(2), Some(NearToken::from_yoctonear(5)));
    /// assert_eq!(NearToken::from_yoctonear(2).checked_div(0), None);
    /// ```
    pub const fn checked_div(self, rhs: u128) -> Option<Self> {
        if let Some(near) = self.as_yoctonear().checked_div(rhs) {
            Some(Self::from_yoctonear(near))
        } else {
            None
        }
    }

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(5).saturating_add(NearToken::from_yoctonear(5)), NearToken::from_yoctonear(10));
    /// assert_eq!(NearToken::from_yoctonear(u128::MAX).saturating_add(NearToken::from_yoctonear(1)), NearToken::from_yoctonear(u128::MAX));
    /// ```
    pub const fn saturating_add(self, rhs: Self) -> Self {
        NearToken::from_yoctonear(self.as_yoctonear().saturating_add(rhs.as_yoctonear()))
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(5).saturating_sub(NearToken::from_yoctonear(2)), NearToken::from_yoctonear(3));
    /// assert_eq!(NearToken::from_yoctonear(1).saturating_sub(NearToken::from_yoctonear(2)), NearToken::from_yoctonear(0));
    /// ```
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        NearToken::from_yoctonear(self.as_yoctonear().saturating_sub(rhs.as_yoctonear()))
    }

    /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// use std::u128;
    /// assert_eq!(NearToken::from_yoctonear(2).saturating_mul(5), NearToken::from_yoctonear(10));
    /// assert_eq!(NearToken::from_yoctonear(u128::MAX).saturating_mul(2), NearToken::from_yoctonear(u128::MAX));
    /// ```
    pub const fn saturating_mul(self, rhs: u128) -> Self {
        NearToken::from_yoctonear(self.as_yoctonear().saturating_mul(rhs))
    }

    /// Saturating integer division. Computes self / rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10).saturating_div(2), NearToken::from_yoctonear(5));
    /// assert_eq!(NearToken::from_yoctonear(10).saturating_div(0), NearToken::from_yoctonear(0))
    /// ```
    pub const fn saturating_div(self, rhs: u128) -> Self {
        if rhs == 0 {
            return NearToken::from_yoctonear(0);
        }
        NearToken::from_yoctonear(self.as_yoctonear().saturating_div(rhs))
    }

    /// Formats the `NearToken` and displays the amount in NEAR or yoctoNEAR depending on the value.
    ///
    /// # Examples
    /// ```
    /// use near_token::NearToken;
    /// assert_eq!(NearToken::from_yoctonear(10_u128.pow(24)).exact_amount_display(), "1 NEAR");
    /// assert_eq!(NearToken::from_yoctonear(15 * 10_u128.pow(23)).exact_amount_display(), "1.5 NEAR");
    /// assert_eq!(NearToken::from_yoctonear(500).exact_amount_display(), "500 yoctoNEAR");
    /// assert_eq!(NearToken::from_yoctonear(0).exact_amount_display(), "0 NEAR");
    /// ```
    pub fn exact_amount_display(&self) -> String {
        let yoctonear = self.as_yoctonear();

        if yoctonear == 0 {
            "0 NEAR".to_string()
        } else if yoctonear <= 1_000 {
            format!("{} yoctoNEAR", yoctonear)
        } else if yoctonear % ONE_NEAR == 0 {
            format!("{} NEAR", yoctonear / ONE_NEAR)
        } else {
            format!(
                "{}.{} NEAR",
                yoctonear / ONE_NEAR,
                format!("{:0>24}", yoctonear % ONE_NEAR).trim_end_matches('0')
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::NearToken;

    #[test]
    fn checked_add_tokens() {
        let tokens = NearToken::from_yoctonear(u128::MAX - 3);
        let any_tokens = NearToken::from_yoctonear(3);
        let more_tokens = NearToken::from_yoctonear(4);
        assert_eq!(
            tokens.checked_add(any_tokens),
            Some(NearToken::from_yoctonear(u128::MAX))
        );
        assert_eq!(tokens.checked_add(more_tokens), None);
    }

    #[test]
    fn checked_sub_tokens() {
        let tokens = NearToken::from_yoctonear(3);
        let any_tokens = NearToken::from_yoctonear(1);
        let more_tokens = NearToken::from_yoctonear(4);
        assert_eq!(
            tokens.checked_sub(any_tokens),
            Some(NearToken::from_yoctonear(2))
        );
        assert_eq!(tokens.checked_sub(more_tokens), None);
    }

    #[test]
    fn checked_mul_tokens() {
        let tokens = NearToken::from_yoctonear(u128::MAX / 10);
        assert_eq!(
            tokens.checked_mul(10),
            Some(NearToken::from_yoctonear(u128::MAX / 10 * 10))
        );
        assert_eq!(tokens.checked_mul(11), None);
    }

    #[test]
    fn checked_div_tokens() {
        let tokens = NearToken::from_yoctonear(10);
        assert_eq!(tokens.checked_div(2), Some(NearToken::from_yoctonear(5)));
        assert_eq!(tokens.checked_div(11), Some(NearToken::from_yoctonear(0)));
        assert_eq!(tokens.checked_div(0), None);
    }

    #[test]
    fn saturating_add_tokens() {
        let tokens = NearToken::from_yoctonear(100);
        let added_tokens = NearToken::from_yoctonear(1);
        let another_tokens = NearToken::from_yoctonear(u128::MAX);
        assert_eq!(
            tokens.saturating_add(added_tokens),
            NearToken::from_yoctonear(101)
        );
        assert_eq!(
            another_tokens.saturating_add(added_tokens),
            NearToken::from_yoctonear(u128::MAX)
        );
    }

    #[test]
    fn saturating_sub_tokens() {
        let tokens = NearToken::from_yoctonear(100);
        let rhs_tokens = NearToken::from_yoctonear(1);
        let another_tokens = NearToken::from_yoctonear(u128::MIN);
        assert_eq!(
            tokens.saturating_sub(rhs_tokens),
            NearToken::from_yoctonear(99)
        );
        assert_eq!(
            another_tokens.saturating_sub(rhs_tokens),
            NearToken::from_yoctonear(u128::MIN)
        );
    }

    #[test]
    fn saturating_mul_tokens() {
        let tokens = NearToken::from_yoctonear(2);
        let rhs = 10;
        let another_tokens = u128::MAX;
        assert_eq!(tokens.saturating_mul(rhs), NearToken::from_yoctonear(20));
        assert_eq!(
            tokens.saturating_mul(another_tokens),
            NearToken::from_yoctonear(u128::MAX)
        );
    }

    #[test]
    fn saturating_div_tokens() {
        let tokens = NearToken::from_yoctonear(10);
        let rhs = 2;
        let another_tokens = 20;
        assert_eq!(tokens.saturating_div(rhs), NearToken::from_yoctonear(5));
        assert_eq!(
            tokens.saturating_div(another_tokens),
            NearToken::from_yoctonear(0)
        );
    }

    #[test]
    fn exact_amount_display_tokens() {
        let token = NearToken::from_yoctonear(0);
        assert_eq!(token.exact_amount_display(), "0 NEAR");

        let token = NearToken::from_yoctonear(500);
        assert_eq!(token.exact_amount_display(), "500 yoctoNEAR");

        let token = NearToken::from_yoctonear(10_u128.pow(24));
        assert_eq!(token.exact_amount_display(), "1 NEAR");

        let token = NearToken::from_yoctonear(15 * 10_u128.pow(23));
        assert_eq!(token.exact_amount_display(), "1.5 NEAR");

        let token = NearToken::from_yoctonear(1_234_567_890_123_456_789_000_000);
        assert_eq!(token.exact_amount_display(), "1.234567890123456789 NEAR");
    }
}
