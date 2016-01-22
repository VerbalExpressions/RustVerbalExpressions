RustVerbalExpressions
=====================

This crate provides a Rust implementation of VerbalExpressions in order to build regex
strings without knowing the minutiae of regex syntax.
It uses the [`regex`](https://github.com/rust-lang-nursery/regex) crate to compile the created regex strings.

[![Build Status](https://travis-ci.org/VerbalExpressions/RustVerbalExpressions.svg?branch=master)](https://travis-ci.org/VerbalExpressions/RustVerbalExpressions)

Versions are numbered according to [semver](http://semver.org/).

# Usage
Add this to your Cargo.toml:
```toml
[dependencies]
verex = "0.1"
```
and this to your crate root:
```rust
extern crate verex;
```

# Examples
A simple example to show the usage:
```rust
extern crate verex;
use verex::Verex;
use verex::find;

fn main() {
    // You can either use a mutable Verex to define different regexes
    let mut verex = Verex::new();
    let regex1 = verex.find("a")
                      .compile()
                      .unwrap();
    let regex2 = verex.or_find("b")
                      .compile()
                      .unwrap();
    // Or just use it for building one (you can use the functions directly as constructors)
    let regex3 = find("a") // or: Verex::new().find("a")
                 .or_find("b")
                 .compile()
                 .unwrap();

    // Test whether the regexes match correctly
    assert!(!regex1.is_match("b"));
    assert!(regex2.is_match("b"));
    assert!(regex3.is_match("b"));
}
```

Here's a URL testing example shamelessly stolen from the python Verex readme:
```rust
extern crate verex;
use verex::start_of_line;

fn main() {
    // Create an example of how to test for correctly formed URLs
    let verex = start_of_line()
                .find("http")
                .maybe("s")
                .find("://")
                .maybe("www.")
                .anything_but(" ")
                .end_of_line();
    let regex = verex.compile().unwrap();
    // Create an example URL
    let test_url = r"https://www.google.com";
    // Test if the URL is valid
    assert!(regex.is_match(test_url));
    // Test the generated regex string
    assert_eq!(verex.source(), r"(?:^(?:http)(?:s)?(?:://)(?:www.)?(?:[^ ]*)$)");
}
```
