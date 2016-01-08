//! This crate provides a Rust implementation of VerbalExpressions in order to build regex
//! strings without knowing the minutiae of regex syntax.
//!
//! It uses the `Regex` crate to compile the created regex strings.
//!
//! # Examples
//!
//! A simple example to show the usage:
//!
//! ```rust
//! # extern crate verex;
//! use verex::VerEx;
//! use verex::find;
//!
//! # fn main() {
//!     // You can either use a mutable verex to define different regexes
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
//!     // Also you can just use the functions directly as constructors
//!     let regex4 = find("a")
//!                  .or_find("b")
//!                  .compile()
//!                  .unwrap();
//!
//!     // Test whether the regexes match correctly
//!     assert!(!regex1.is_match("b"));
//!     assert!(regex2.is_match("b"));
//!     assert!(regex3.is_match("b"));
//!     assert!(regex4.is_match("b"));
//!
//!     // Test the generated regex strings
//!     assert_eq!(regex1.as_str(), r"(?:a)");
//!     assert_eq!(regex2.as_str(), r"(?:a)|(?:b)");
//!     assert_eq!(regex3.as_str(), r"(?:a)|(?:b)");
//!     assert_eq!(regex4.as_str(), r"(?:a)|(?:b)");
//! # }
//! ```
//!
//!
//! Here's a URL testing example shamelessly stolen from the python VerEx readme:
//!
//! ```rust
//! # extern crate verex;
//! use verex::VerEx;
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
//!
//! Example usage of the or! macro:
//!
//! ```rust
//! #[macro_use(or)]
//! extern crate verex;
//!
//! # fn main() {
//!     let regex = or!("foo", "bar", "baz")
//!                 .compile()
//!                 .unwrap();
//!
//!     // Test if the regex matches correctly
//!     assert!(regex.is_match("foo"));
//!     assert!(regex.is_match("bar"));
//!     assert!(regex.is_match("baz"));
//!     assert!(!regex.is_match("bum"));
//!
//!     // Test the generated regex string
//!     assert_eq!(regex.as_str(), r"(?:foo)|(?:bar)|(?:baz)");
//! # }
//! ```

#![warn(missing_docs)]

extern crate regex;

pub use verex::VerEx;

mod verex;

// standalone functions
/// Any of the given characters
pub fn any(chars: &str) -> VerEx {
    VerEx::new().any(chars).clone()
}

/// See any()
pub fn any_of(chars: &str) -> VerEx {
    any(chars)
}

/// Any character zero or more times
pub fn anything() -> VerEx {
    VerEx::new().anything().clone()
}

/// Any character zero or more times except the provided characters
pub fn anything_but(value: &str) -> VerEx {
    VerEx::new().anything_but(value).clone()
}

/// A line break!
pub fn br() -> VerEx {
    line_break()
}

/// Find a specific string and capture it
pub fn capture(value: &str) -> VerEx {
    VerEx::new().capture(value).clone()
}

/// Add a token for the end of a line
pub fn end_of_line() -> VerEx {
    VerEx::new().end_of_line().clone()
}

/// Find a specific string
pub fn find(value: &str) -> VerEx {
    VerEx::new().find(value).clone()
}

/// A line break!
pub fn line_break() -> VerEx {
    VerEx::new().line_break().clone()
}

/// Any string either one or zero times
pub fn maybe(value: &str) -> VerEx {
    VerEx::new().maybe(value).clone()
}

/// Match any of the given sub-expressions
#[macro_export]
macro_rules! or {
    ( $first_string:expr, $( $string:expr ),* ) => {
        {
            let mut verex = $crate::VerEx::new();
            verex.find($first_string);
            $(
                verex.or_find($string);
            )*
            verex
        }
    };
}

/// A range of characters e.g. [A-Z]
/// Usage example: verex.range(vec![('a', 'z'),('A', 'Z')])
pub fn range(range: Vec<(char, char)>) -> VerEx {
    VerEx::new().range(range).clone()
}

/// Any character at least one time
pub fn something() -> VerEx {
    VerEx::new().something().clone()
}

/// Any character at least one time except for these characters
pub fn something_but(value: &str) -> VerEx {
    VerEx::new().something_but(value).clone()
}

/// Add a token for the start of a line
pub fn start_of_line() -> VerEx {
    VerEx::new().start_of_line().clone()
}

/// Add a token for a tab
pub fn tab() -> VerEx {
    VerEx::new().tab().clone()
}

/// Any alphanumeric characters
pub fn word() -> VerEx {
    VerEx::new().word().clone()
}
