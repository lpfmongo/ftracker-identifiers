//! CNPJ (Cadastro Nacional da Pessoa Jurídica). Brazil's national registry identifier for legal entities.
//!
//! This module provides the validated Rust representation and the parsing, formatting, validation,
//! and error types that surround it. It accepts both conventional formatted input and the compact
//! 14-character form, normalizes ASCII case, and guarantees that any constructed [`Cnpj`] satisfies
//! the format and checksum rules.
//!
//! # What this type represents
//!
//! A CNPJ has 14 meaningful characters:
//!
//! - the first 12 identify the entity root plus branch/order;
//! - the last two are verification digits.
//!
//! [`Cnpj`] stores those characters in normalized uppercase ASCII and exposes borrowed accessors for
//! both the compact and punctuated forms.
//!
//! # Numeric and alphanumeric formats
//!
//! The public format changed in 2026: the first 12 positions may contain uppercase letters as well
//! as digits, while the final two verification digits remain numeric. Older numeric-only CNPJs remain
//! valid and are treated as a special case of the same 14-character, same-checksum format. [`Cnpj`]
//! represents both uniformly; there is no separate legacy type.
//!
//! # Examples
//!
//! ```
//! use ftracker_identifiers::Cnpj;
//!
//! let numeric = Cnpj::parse("00.000.000/0001-91").unwrap();
//! assert!(numeric.is_root());
//! assert_eq!(numeric.as_str(), "00000000000191");
//! assert_eq!(numeric.formatted().as_str(), "00.000.000/0001-91");
//!
//! let alpha = Cnpj::parse("12ABC34501DE35").unwrap();
//! assert_eq!(alpha.branch_code(), "01DE");
//! assert_eq!(alpha.branch_number(), None);
//! ```

mod error;
mod fmt;
mod parser;
mod validation;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "schemars")]
mod schema;

#[cfg(feature = "arbitrary")]
mod arbitrary;

#[cfg(any(test, feature = "proptest"))]
pub mod proptest;

#[cfg(test)]
mod tests;

pub use error::CnpjError;
pub use fmt::FormattedCnpj;

use core::convert::TryFrom;
use core::str::{FromStr, from_utf8_unchecked};

/// A validated CNPJ (Cadastro Nacional da Pessoa Jurídica).
///
/// `Cnpj` is a 14-byte, `Copy`, allocation-free value object. Once constructed, it is guaranteed to
/// satisfy the structural rules and Módulo 11 checksum required by the crate.
///
/// Internally, the identifier is stored as raw uppercase ASCII bytes (`'0'...='9'` or `'A'...='Z'`).
/// This keeps the compact representation lossless and makes borrowed access to the normalized form cheap.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cnpj {
    bytes: [u8; 14],
}

impl Cnpj {
    /// Parses a CNPJ from a string.
    ///
    /// The parser accepts the conventional `AA.AAA.AAA/AAAA-DD` form as well as the compact
    /// 14-character form. It also tolerates surrounding and embedded ASCII spaces and folds ASCII
    /// letters to uppercase before validation.
    ///
    /// This is the primary constructor; [`Cnpj::new`], [`FromStr`], and [`TryFrom<&str>`] all delegate to it.
    ///
    /// # Errors
    ///
    /// Returns [`CnpjError`] if the input is empty, does not contain exactly 14 meaningful
    /// characters after formatting is removed, contains a character invalid for its position,
    /// consists of a single repeated character, or fails the checksum.
    pub fn parse(input: &str) -> Result<Self, CnpjError> {
        let candidate = parser::normalize(input)?;
        Self::from_bytes(candidate)
    }

    /// Alias for [`Cnpj::parse`].
    #[inline]
    pub fn new(input: &str) -> Result<Self, CnpjError> {
        Self::parse(input)
    }

    /// Constructs a `Cnpj` directly from 14 raw ASCII bytes.
    ///
    /// Each byte must already be an ASCII digit, and for the first 12 positions may also be an
    /// uppercase ASCII letter. Use [`Cnpj::parse`] if the input might contain punctuation or
    /// lowercase letters.
    ///
    /// Numeric-only CNPJs remain fully supported. Pass ASCII digit bytes (`b'0'...=b'9'`), not raw
    /// numeric values.
    #[doc(alias = "from_digits")]
    pub fn from_bytes(bytes: [u8; 14]) -> Result<Self, CnpjError> {
        validation::validate(&bytes)?;
        Ok(Cnpj { bytes })
    }

    /// Returns the 14 raw ASCII bytes backing this CNPJ.
    ///
    /// The returned bytes are in compact form, without punctuation (for example, `b"12ABC34501DE35"`).
    #[doc(alias = "digits")]
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 14] {
        &self.bytes
    }

    /// Returns the compact CNPJ as a `&str`.
    ///
    /// This never allocates: the bytes are guaranteed to be valid ASCII by construction.
    #[inline]
    pub fn as_str(&self) -> &str {
        // SAFETY: The bytes array is strictly guaranteed to contain only
        // valid ASCII uppercase alphanumeric characters by `Cnpj::from_bytes`.
        unsafe { from_utf8_unchecked(&self.bytes) }
    }

    /// Renders the punctuated `AA.AAA.AAA/AAAA-DD` form without heap allocation.
    ///
    /// See [`FormattedCnpj`].
    #[inline]
    pub fn formatted(&self) -> FormattedCnpj {
        FormattedCnpj::new(self)
    }

    /// Returns the 8-character root segment.
    ///
    /// This identifies the entity itself and is shared by the company and all of its branches.
    #[inline]
    pub fn root(&self) -> &str {
        &self.as_str()[0..8]
    }

    /// Returns the 4-character branch/order segment.
    ///
    /// `"0001"` conventionally denotes the head office (matriz); see [`Cnpj::is_root`].
    #[inline]
    pub fn branch_code(&self) -> &str {
        &self.as_str()[8..12]
    }

    /// Returns `true` when the branch/order segment is `"0001"`.
    #[inline]
    pub fn is_root(&self) -> bool {
        self.branch_code() == "0001"
    }

    /// Returns the branch/order segment as a number when it is purely numeric.
    ///
    /// Returns `None` when the segment contains a letter, which is only possible for
    /// alphanumeric-format CNPJs. Numeric CNPJs, including the conventional matriz marker (`"0001"`),
    /// always parse successfully.
    pub fn branch_number(&self) -> Option<u16> {
        self.branch_code().parse().ok()
    }

    /// Returns the two verification digits as numeric values.
    #[inline]
    pub fn check_digits(&self) -> (u8, u8) {
        (self.bytes[12] - b'0', self.bytes[13] - b'0')
    }
}

impl FromStr for Cnpj {
    type Err = CnpjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for Cnpj {
    type Error = CnpjError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl AsRef<[u8]> for Cnpj {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<str> for Cnpj {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
