#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![doc(html_logo_url = "https://gist.github.com/user-attachments/assets/8dc46e20-90d6-431a-8805-665549e02650")]

/// See the [`crate`] top-level docs for more info.
pub use ::negative_proc_macros::negative_impl;

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}
