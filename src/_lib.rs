#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![doc(html_logo_url = "https://gist.github.com/user-attachments/assets/97d0d1a5-538c-4f8d-86df-72e8b2ca14cc")]

/// See the [`crate`] top-level docs for more info.
pub use ::negative_proc_macros::negative_impl;

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}
