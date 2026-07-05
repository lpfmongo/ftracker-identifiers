#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod cnpj;
pub use cnpj::{Cnpj, CnpjError, FormattedCnpj};

#[cfg(feature = "proptest")]
#[doc(hidden)]
pub use cnpj::proptest;
