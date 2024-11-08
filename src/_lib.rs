#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![doc(html_logo_url = "https://gist.github.com/user-attachments/assets/d0dc0b98-aa31-4078-8164-ae6f04d4b6e7")]

/// See the [`crate`] top-level docs for more info.
pub use ::negative_proc_macros::negative_impl;

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}
