//! This crate provides a Rust implementation of VerbalExpressions in order to build regex
//! strings without knowing the minutiae of regex syntax.
//!
//! It uses the `Regex` crate to compile the created regex strings.
//!
//! # Examples
//!
//! Here's a simple example shamelessly stolen from the python VerEx readme:
//!
//! ```rust
//! extern crate verex;
//!
//! use verex::VerEx;
//!
//! fn main() {
//!     // Create an example of how to test for correctly formed URLs
//!     let mut verex = VerEx::new();
//!     let regex = verex
//!                 .start_of_line()
//!                 .find("http")
//!                 .maybe("s")
//!                 .find("://")
//!                 .maybe("www.")
//!                 .anything_but(" ")
//!                 .end_of_line()
//!                 .compile()
//!                 .unwrap();
//!
//!     // Create an example URL
//!     let test_url = r"https://www.google.com";
//!
//!     // Test if the URL is valid
//!     assert!(regex.is_match(test_url));
//!
//!     // Print the generated regex
//!     assert_eq!(verex.source(), r"^(?:http)(?:s)?(?:://)(?:www.)?(?:[^ ]*)$")
//! }
//! ```

#![warn(missing_docs)]

extern crate regex;

pub use verex::VerEx;

mod verex;
