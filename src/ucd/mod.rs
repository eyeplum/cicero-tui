// This file is part of Cicero.
//
// Cicero is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Cicero is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Cicero. If not, see <https://www.gnu.org/licenses/>.

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
