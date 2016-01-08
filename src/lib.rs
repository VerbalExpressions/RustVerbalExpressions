//! This crate provides a Rust implementation of VerbalExpressions in order to build regex
//! strings without knowing the minutiae of regex syntax.
//!
//! It uses the `Regex` crate to compile the created regex strings.
//!
//! # Examples
//!
//! ```rust
//! # extern crate verex; use verex::VerEx;
//!
//! # fn main() {
//!     // You can either use a mutable verx to define different regexes
//!     let mut verex = VerEx::new();
//!     let regex1 = verex.find("a")
//!                       .compile()
//!                       .unwrap();
//!
//!     let regex2 = verex.or_find("b")
//!                       .compile()
//!                       .unwrap();
//!
//!     // Or just use it for building one
//!     let regex3 = VerEx::new().find("a")
//!                              .or_find("b")
//!                              .compile()
//!                              .unwrap();
//!
//!     // Test if the URL is valid
//!     assert!(!regex1.is_match("b"));
//!     assert!(regex2.is_match("b"));
//!     assert!(regex3.is_match("b"));
//!
//!     // Test the generated regex string
//!     assert_eq!(regex1.as_str(), r"(?:a)");
//!     assert_eq!(regex2.as_str(), r"(?:a)|(?:b)");
//!     assert_eq!(regex3.as_str(), r"(?:a)|(?:b)");
//! # }
//! ```
//!
//!
//! Here's a URL testing example shamelessly stolen from the python VerEx readme:
//!
//! ```rust
//! # extern crate verex; use verex::VerEx;
//!
//! # fn main() {
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
//!     // Test the generated regex string
//!     assert_eq!(verex.source(), r"^(?:http)(?:s)?(?:://)(?:www.)?(?:[^ ]*)$");
//! # }
//! ```

#![warn(missing_docs)]

extern crate regex;

pub use verex::VerEx;

mod verex;
