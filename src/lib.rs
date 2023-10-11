//! A `NearGas` type to represent a value of Gas.
//!
//! Each `NearGas` is composed of a whole number of Gases.
//! `NearGas` is implementing the common trait `FromStr`. Also, have utils function to parse from `str` into `u64`.
//!
//! # Examples
//! ```
//! use near_gas::*;
//!
//! let one_tera_gas = NearGas::from_gas(10u64.pow(12));
//! assert_eq!(one_tera_gas, NearGas::from_tgas(1u64));
//! assert_eq!(one_tera_gas, NearGas::from_ggas(1000u64));
//! ```
//!
//! # Crate features
//!
//! * **borsh** (optional) -
//!   When enabled allows `NearGas` to serialized and deserialized by `borsh`.
//!
//! * **serde** (optional) -
//!   When enabled allows `NearGas` to serialized and deserialized by `serde`.
//!
//! * **schemars** (optional) -
//!  Implements `schemars::JsonSchema` for `NearGas`.
//!
//! * **interactive-clap** (optional) -
//!  Implements `interactive_clap::ToCli` for `NearGas`.
mod error;

mod utils;

mod trait_impls;

pub use self::error::NearTokenError;
pub use self::utils::DecimalNumberParsingError;


#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize, borsh::BorshSchema)
)]
#[repr(transparent)]
pub struct NearToken {
    inner: u128,
}

const ONE_NEAR:u128 = 10u128.pow(24);
const ONE_MILI_NEAR:u128 = 10u128.pow(21);

impl NearToken {
    pub const  fn from_yoctonear(inner: u128) -> Self {
        Self { inner }
    }
    
    pub const fn from_mili_near(inner: u128) -> Self {
        Self { inner: inner * ONE_MILI_NEAR }
    }

    pub const fn from_near(inner: u128) -> Self{
        Self {inner: inner * ONE_NEAR}
    } 

    pub const fn as_near(&self) -> u128 {
        self.inner / ONE_NEAR
    }

    pub const  fn as_yoctonear(&self) -> u128 {
        self.inner
    }
   
    pub const fn as_mili_near(&self) -> u128 {
        self.inner / ONE_MILI_NEAR
    }

    pub const fn is_zero(&self) -> bool {
        self.inner == 0
    }

    /// Checked integer addition. Computes self + rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(u64::MAX -2).checked_add(NearGas::from_gas(2)), Some(NearGas::from_gas(u64::MAX)));
    /// assert_eq!(NearGas::from_gas(u64::MAX -2).checked_add(NearGas::from_gas(3)), None);
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
    /// use near_gas::NearGas;
    /// assert_eq!(NearGas::from_gas(2).checked_sub(NearGas::from_gas(2)), Some(NearGas::from_gas(0)));
    /// assert_eq!(NearGas::from_gas(2).checked_sub(NearGas::from_gas(3)), None);
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
    /// use near_gas::NearGas;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(2).checked_mul(2), Some(NearGas::from_gas(4)));
    /// assert_eq!(NearGas::from_gas(u64::MAX).checked_mul(2), None)
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
    /// use near_gas::NearGas;
    /// assert_eq!(NearGas::from_gas(10).checked_div(2), Some(NearGas::from_gas(5)));
    /// assert_eq!(NearGas::from_gas(2).checked_div(0), None);
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
    /// use near_gas::NearGas;
    /// assert_eq!(NearGas::from_gas(5).saturating_add(NearGas::from_gas(5)), NearGas::from_gas(10));
    /// assert_eq!(NearGas::from_gas(u64::MAX).saturating_add(NearGas::from_gas(1)), NearGas::from_gas(u64::MAX));
    /// ```
    pub const fn saturating_add(self, rhs: Self) -> Self {
        NearToken::from_yoctonear(self.as_yoctonear().saturating_add(rhs.as_yoctonear()))
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// assert_eq!(NearGas::from_gas(5).saturating_sub(NearGas::from_gas(2)), NearGas::from_gas(3));
    /// assert_eq!(NearGas::from_gas(1).saturating_sub(NearGas::from_gas(2)), NearGas::from_gas(0));
    /// ```
    pub const fn saturating_sub(self, rhs: Self) -> Self{
        NearToken::from_yoctonear(self.as_yoctonear().saturating_sub(rhs.as_yoctonear()))
    }

    /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(2).saturating_mul(5), NearGas::from_gas(10));
    /// assert_eq!(NearGas::from_gas(u64::MAX).saturating_mul(2), NearGas::from_gas(u64::MAX));
    /// ```
    pub const fn saturating_mul(self, rhs: u128) -> Self{
        NearToken::from_yoctonear(self.as_yoctonear().saturating_mul(rhs))
    }

    /// Saturating integer division. Computes self / rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// assert_eq!(NearGas::from_gas(10).saturating_div(2), NearGas::from_gas(5));
    /// assert_eq!(NearGas::from_gas(10).saturating_div(0), NearGas::from_gas(0))
    /// ```
    pub const fn saturating_div(self, rhs: u128) -> Self {
        if rhs == 0 {
            return NearToken::from_yoctonear(0);
        }
        NearToken::from_yoctonear(self.as_yoctonear().saturating_div(rhs))
    }
}

#[cfg(test)]
mod test {
    use crate::NearToken;

    #[test]
    fn checked_add_gas() {
        let gas = NearToken::from_yoctonear(u128::MAX - 3);
        let any_gas = NearToken::from_yoctonear(3);
        let more_gas = NearToken::from_yoctonear(4);
        assert_eq!(gas.checked_add(any_gas), Some(NearToken::from_yoctonear(u128::MAX)));
        assert_eq!(gas.checked_add(more_gas), None);
    }

    #[test]
    fn checked_sub_gas() {
        let gas = NearToken::from_yoctonear(3);
        let any_gas = NearToken::from_yoctonear(1);
        let more_gas = NearToken::from_yoctonear(4);
        assert_eq!(gas.checked_sub(any_gas), Some(NearToken::from_yoctonear(2)));
        assert_eq!(gas.checked_sub(more_gas), None);
    }

    #[test]
    fn checked_mul_gas() {
        let gas = NearToken::from_yoctonear(u128::MAX / 10);
        assert_eq!(
            gas.checked_mul(10),
            Some(NearToken::from_yoctonear(u128::MAX / 10 * 10))
        );
        assert_eq!(gas.checked_mul(11), None);
    }

    #[test]
    fn checked_div_gas() {
        let gas = NearToken::from_yoctonear(10);
        assert_eq!(gas.checked_div(2), Some(NearToken::from_yoctonear(5)));
        assert_eq!(gas.checked_div(11), Some(NearToken::from_yoctonear(0)));
        assert_eq!(gas.checked_div(0), None);
    }

    #[test]
    fn saturating_add_gas() {
        let gas = NearToken::from_yoctonear(100);
        let added_gas = NearToken::from_yoctonear(1);
        let another_gas = NearToken::from_yoctonear(u128::MAX);
        assert_eq!(
            gas.saturating_add(added_gas.clone()),
            NearToken::from_yoctonear(101)
        );
        assert_eq!(
            another_gas.saturating_add(added_gas),
            NearToken::from_yoctonear(u128::MAX)
        );
    }

    #[test]
    fn saturating_sub_gas() {
        let gas = NearToken::from_yoctonear(100);
        let rhs_gas = NearToken::from_yoctonear(1);
        let another_gas = NearToken::from_yoctonear(u128::MIN);
        assert_eq!(gas.saturating_sub(rhs_gas.clone()), NearToken::from_yoctonear(99));
        assert_eq!(
            another_gas.saturating_sub(rhs_gas),
            NearToken::from_yoctonear(u128::MIN)
        );
    }

    #[test]
    fn saturating_mul_gas() {
        let gas = NearToken::from_yoctonear(2);
        let rhs = 10;
        let another_gas = u128::MAX;
        assert_eq!(gas.saturating_mul(rhs), NearToken::from_yoctonear(20));
        assert_eq!(gas.saturating_mul(another_gas), NearToken::from_yoctonear(u128::MAX));
    }

    #[test]
    fn saturating_div_gas() {
        let gas = NearToken::from_yoctonear(10);
        let rhs = 2;
        let another_gas = 20;
        assert_eq!(gas.saturating_div(rhs), NearToken::from_yoctonear(5));
        assert_eq!(gas.saturating_div(another_gas), NearToken::from_yoctonear(0));
    }
}
