#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod cnpj;
pub use cnpj::Cnpj;

#[cfg(feature = "proptest")]
pub use cnpj::proptest;
