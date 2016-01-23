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
//! use verex::Verex;
//! use verex::find;
//!
//! # fn main() {
//!     // You can either use a mutable Verex to define different regexes
//!     let mut verex = Verex::new();
//!     let regex1 = verex.find("a")
//!                       .compile()
//!                       .unwrap();
//!
//!     let regex2 = verex.or_find("b")
//!                       .compile()
//!                       .unwrap();
//!
//!     // Or just use it for building one (you can use the funcitons directly as constructors)
//!     let regex3 = find("a") // or: Verex::new().find("a")
//!                  .or_find("b")
//!                  .compile()
//!                  .unwrap();
//!
//!     // Test whether the regexes match correctly
//!     assert!(!regex1.is_match("b"));
//!     assert!(regex2.is_match("b"));
//!     assert!(regex3.is_match("b"));
//!
//!     // Test the generated regex strings
//!     assert_eq!(regex1.as_str(), r"(?:(?:a))");
//!     assert_eq!(regex2.as_str(), r"(?:(?:a)|(?:b))");
//!     assert_eq!(regex3.as_str(), r"(?:(?:a)|(?:b))");
//! # }
//! ```
//!
//!
//! Here's a URL testing example shamelessly stolen from the python Verex readme:
//!
//! ```rust
//! # extern crate verex;
//! use verex::start_of_line;
//!
//! # fn main() {
//!     // Create an example of how to test for correctly formed URLs
//!     let verex = start_of_line()
//!                 .find("http")
//!                 .maybe("s")
//!                 .find("://")
//!                 .maybe("www.")
//!                 .anything_but(" ")
//!                 .end_of_line()
//!                 .clone();
//!     let regex = verex.compile().unwrap();
//!
//!     // Create an example URL
//!     let test_url = r"https://www.google.com";
//!
//!     // Test if the URL is valid
//!     assert!(regex.is_match(test_url));
//!
//!     // Test the generated regex string
//!     assert_eq!(verex.source(), r"(?:^(?:http)(?:s)?(?:://)(?:www\.)?(?:[^ ]*)$)");
//! # }
//! ```
//!
//! Example usage of the `or!` macro:
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
//!     assert_eq!(regex.as_str(), r"(?:(?:foo)|(?:bar)|(?:baz))");
//! # }
//! ```
//!
//! Example usage of the `or_expr!` macro:
//!
//! ```rust
//! #[macro_use(or_expr)]
//! extern crate verex;
//! extern crate regex;
//! use verex::Expression as E;
//! use verex::Verex;
//! use regex::Regex;
//!
//! # fn main() {
//!     let sub_verex = Verex::from_str("Darth(Vader)*?");
//!     let sub_regex = Regex::new("(?P<robot>C3PO)").unwrap();
//!     let regex = or_expr!(
//!                     E::String("([RD]2){2}"),
//!                     E::Verex(&sub_verex),
//!                     E::Regex(&sub_regex))
//!                 .compile()
//!                 .unwrap();
//!
//!     // Test if the regex matches correctly
//!     assert!(regex.is_match("R2D2"));
//!     assert!(regex.is_match("Darth"));
//!     assert!(regex.is_match("C3PO"));
//!     assert!(!regex.is_match("Anakin"));
//!
//!     // Test the generated regex string
//!     assert_eq!(regex.as_str(), r"(?:(?:([RD]2){2})|(?:(?:Darth(Vader)*?))|(?:(?P<robot>C3PO)))");
//! # }
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate bitflags;
extern crate regex;

pub use verex::Verex;
pub use verex::Expression;

mod verex;

// standalone functions
/// Any of the given characters
pub fn any(chars: &str) -> Verex {
    Verex::new().any(chars).clone()
}

/// See any()
pub fn any_of(chars: &str) -> Verex {
    any(chars)
}

/// Any character zero or more times
pub fn anything() -> Verex {
    Verex::new().anything().clone()
}

/// Any character zero or more times except the provided characters
pub fn anything_but(chars: &str) -> Verex {
    Verex::new().anything_but(chars).clone()
}

/// A line break!
pub fn br() -> Verex {
    line_break()
}

/// Find a specific string and capture it (will get escaped)
pub fn capture(value: &str) -> Verex {
    Verex::new().capture(value).clone()
}

/// Find an expression and capture it
pub fn capture_expr(expr: Expression) -> Verex {
    Verex::new().capture_expr(expr).clone()
}

/// Add the token for matching digits
pub fn digit() -> Verex {
    Verex::new().digit().clone()
}

/// Add a token for the end of a line
pub fn end_of_line() -> Verex {
    Verex::new().end_of_line().clone()
}

/// Find a specific string
pub fn find(value: &str) -> Verex {
    Verex::new().find(value).clone()
}

/// Find an expression
pub fn find_expr(expr: Expression) -> Verex {
    Verex::new().find_expr(expr).clone()
}

/// A line break!
pub fn line_break() -> Verex {
    Verex::new().line_break().clone()
}

/// Any string either one or zero times
pub fn maybe(value: &str) -> Verex {
    Verex::new().maybe(value).clone()
}

/// Any string either one or zero times
pub fn maybe_expr(expr: Expression) -> Verex {
    Verex::new().maybe_expr(expr).clone()
}

/// Match any of the given values
#[macro_export]
macro_rules! or {
    ( $first_string:expr, $( $string:expr ),* ) => {
        {
            let mut verex = $crate::Verex::new();
            verex.find($first_string);
            $(
                verex.or_find($string);
            )*
            verex
        }
    };
}

/// Match any of the given sub-expressions
#[macro_export]
macro_rules! or_expr {
    ( $first_e:expr, $( $e:expr ),* ) => {
        {
            let mut verex = $crate::Verex::new();
            verex.find_expr($first_e);
            $(
                verex.or_find_expr($e);
            )*
            verex
        }
    };
}

/// A range of characters e.g. [A-Z]
/// Usage example: verex.range(vec![('a', 'z'),('A', 'Z')])
pub fn range(range: Vec<(char, char)>) -> Verex {
    Verex::new().range(range).clone()
}

/// Toggle whether ^ and $ match line start and end or string start and end
pub fn search_one_line(enable: bool) -> Verex {
    Verex::new().search_one_line(enable).clone()
}

/// Any character at least one time
pub fn something() -> Verex {
    Verex::new().something().clone()
}

/// Any character at least one time except for these characters
pub fn something_but(chars: &str) -> Verex {
    Verex::new().something_but(chars).clone()
}

/// Add a token for the start of a line
pub fn start_of_line() -> Verex {
    Verex::new().start_of_line().clone()
}

/// Add a token for a tab
pub fn tab() -> Verex {
    Verex::new().tab().clone()
}

/// Toggle whether to match case-sensitively or not
pub fn with_any_case(enable: bool) -> Verex {
    Verex::new().with_any_case(enable).clone()
}

/// Any alphanumeric characters
pub fn word() -> Verex {
    Verex::new().word().clone()
}
