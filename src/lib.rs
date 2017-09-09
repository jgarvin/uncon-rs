//! Traits for unchecked conversions between types.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unchecked_convert = "0.1.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate unchecked_convert;
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! A type may wrap around another but must maintain certain invariants that
//! aren't met by the inner type. An example of this is `str` in relation to
//! `[u8]` where a string is just a UTF-8 encoded byte slice.
//!
//! In this example, `U4` is a simple wrapper around `u8` where valid instances
//! must only ever have 4 bits set.
//!
//! ```
//! # use unchecked_convert::*;
//! struct U4(u8);
//!
//! impl From<u8> for U4 {
//!     fn from(byte: u8) -> U4 {
//!         U4(byte & 0b1111)
//!     }
//! }
//!
//! impl FromUnchecked<u8> for U4 {
//!     unsafe fn from_unchecked(byte: u8) -> U4 {
//!         U4(byte)
//!     }
//! }
//! ```
//!
//! If a type `T` implements [`FromUnchecked`] for some type `U`, then `U`
//! automatically implements [`IntoUnchecked`] for `T`.
//!
//! ```
//! # use unchecked_convert::*;
//! let b = [b'h', b'i'];
//! let s: &str = unsafe { b.as_ref().into_unchecked() };
//!
//! assert_eq!(s, "hi");
//! ```
//!
//! # Deriving Traits
//!
//! Deriving traits requires adding the following to your project's
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unchecked_convert_derive = "0.1.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! #[macro_use]
//! extern crate unchecked_convert_derive;
//! # fn main() {}
//! ```
//!
//! The [`FromUnchecked`] trait can be derived for:
//!
//! - Structs with a single field
//! - C-like enums with `#[repr]` attribute
//!
//! ```
//! # extern crate core;
//! # #[macro_use]
//! # extern crate unchecked_convert_derive;
//! # extern crate unchecked_convert;
//! # use unchecked_convert::*;
//! #[derive(FromUnchecked)]
//! struct U4 { bits: u8 }
//!
//! #[derive(FromUnchecked)]
//! #[repr(u8)]
//! enum Flag {
//!     A, B, C, D
//! }
//!
//! # fn main() {
//! unsafe {
//!     let b = 0b1010;
//!     let x = U4::from_unchecked(b);
//!     assert_eq!(x.bits, b);
//!
//!     let n = 2;
//!     let f = Flag::from_unchecked(n);
//!     assert_eq!(n, f as u8);
//! }
//! # }
//! ```
//!
//! [crate]: https://crates.io/crates/unchecked_convert
//! [`FromUnchecked`]: trait.FromUnchecked.html
//! [`IntoUnchecked`]: trait.IntoUnchecked.html

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(feature = "std")]
extern crate core;
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::{String, Vec};

use core::str;

/// Unchecked and potentially unsafe conversions from `T` into `Self`.
pub trait FromUnchecked<T>: Sized {
    /// Performs the unchecked conversion.
    unsafe fn from_unchecked(T) -> Self;
}

/// Unchecked and potentially unsafe conversions from `Self` into `T`.
pub trait IntoUnchecked<T>: Sized {
    /// Performs the unchecked conversion.
    unsafe fn into_unchecked(self) -> T;
}

impl<T, U: FromUnchecked<T>> IntoUnchecked<U> for T {
    #[inline]
    unsafe fn into_unchecked(self) -> U {
        U::from_unchecked(self)
    }
}

impl<'a, T, U> FromUnchecked<&'a U> for &'a T {
    #[inline]
    unsafe fn from_unchecked(other: &U) -> &T {
        &*(other as *const U as *const T)
    }
}

impl<'a, T, U> FromUnchecked<&'a mut U> for &'a mut T {
    #[inline]
    unsafe fn from_unchecked(other: &mut U) -> &mut T {
        &mut *(other as *mut U as *mut T)
    }
}

impl<'a> FromUnchecked<&'a [u8]> for &'a str {
    #[inline]
    unsafe fn from_unchecked(utf8: &[u8]) -> &str {
        str::from_utf8_unchecked(utf8)
    }
}

impl<'a> FromUnchecked<&'a mut [u8]> for &'a mut str {
    #[inline]
    unsafe fn from_unchecked(utf8: &mut [u8]) -> &mut str {
        str::from_utf8_unchecked_mut(utf8)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl FromUnchecked<Vec<u8>> for String {
    #[inline]
    unsafe fn from_unchecked(utf8: Vec<u8>) -> String {
        String::from_utf8_unchecked(utf8)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl FromUnchecked<Box<[u8]>> for String {
    #[inline]
    unsafe fn from_unchecked(utf8: Box<[u8]>) -> String {
        utf8.into_vec().into_unchecked()
    }
}
