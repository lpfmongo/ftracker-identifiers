#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod cnpj;
#[doc(inline)]
pub use cnpj::{Cnpj, CnpjError, FormattedCnpj};

#[cfg(feature = "proptest")]
#[doc(hidden)]
pub use cnpj::proptest;
