#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod cnpj;
#[doc(inline)]
pub use cnpj::{Cnpj, CnpjError, FormattedCnpj};

pub mod isin;
#[doc(inline)]
pub use isin::{Isin, IsinError};
