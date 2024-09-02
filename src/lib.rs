//! **windows-elevate** is a Windows-only crate, it provides utility two functions:
//! - `check_elevated()` is used to determine whether the current process is running as elevated.
//! - `elevate()` is used to elevate the current process permissions.
//!
//! ## Example
//! ```rust
//! use windows_elevate::{check_elevated, elevate};
//!
//!fn test_elevate() {
//!    let is_elevated = check_elevated().expect("Failed to call check_elevated");
//!
//!    if !is_elevated {
//!        elevate().expect("Failed to elevate");
//!        return;
//!    }
//!    // From here, it's the new elevated process
//!}
//! ```
//!
//!
#![cfg(windows)]

mod check_elevated;
mod elevate;

pub use check_elevated::check_elevated;
pub use elevate::elevate;
pub use windows_result;
