//!
//! This module supplements `unic::ucd` by providing missing APIs or wrapper APIs for accessing the
//! Unicode Character Database.
//!
//! Some APIs in this module works with Unicode code points (as `u32`) instead of the built-in
//! `char` because those APIs are intended to also work with isolated surrogate code points -
//! Cicero is a Unicode tool after all, so sometimes it needs to work with all aspects of Unicode.
//!

mod plane;

pub use plane::Plane;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Range {
    start: u32,
    end: u32,
}
