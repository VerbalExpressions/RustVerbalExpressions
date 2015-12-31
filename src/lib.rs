extern crate regex;

use regex::Regex;
use regex::Error;

pub struct VerEx {
    string: String
}

impl VerEx {
    pub fn new() -> VerEx {
        VerEx {
            string: String::new(),
        }
    }

    pub fn from_string(string: String) -> VerEx {
        VerEx {
            string: string,
        }
    }

    pub fn from_str(string: &str) -> VerEx {
        VerEx::from_string(string.to_string())
    }

    pub fn add(&mut self, value: &str) -> &mut VerEx{
        self.string.push_str(value);
        self
    }

    pub fn compile(&mut self) -> Result<Regex, Error> {
        Regex::new(self.string.as_ref())
    }

    pub fn regex(&mut self) -> Result<Regex, Error> {
        self.compile()
    }

    pub fn source(& self) -> &str {
        self.string.as_ref()
    }

    pub fn raw(& self) -> &str {
        self.source()
    }

    // --------------------------------------------------

    pub fn anything(&mut self) -> &mut VerEx {
        self.add(r"(.*)")
    }

    pub fn anything_but(&mut self, value: &str) -> &mut VerEx {
        let mut string = r"([^".to_string();
        string.push_str(value);
        string.push_str(r"]*)");
        self.add(string.as_ref())
    }

    pub fn end_of_line(&mut self) -> &mut VerEx {
        self.add(r"$")
    }

    pub fn start_of_line(&mut self) -> &mut VerEx {
        self.add(r"^")
    }
}

#[test]
fn test_constructors() {
    let verex1: VerEx = VerEx::new();

    let verex2: VerEx = VerEx::from_string(r"a".to_string());

    let verex3: VerEx = VerEx::from_str(r"a");
}

#[test]
fn test_add() {
    let mut verex: VerEx = VerEx::new();
    verex.add(r"a");
    assert_eq!(verex.source(), r"a");
}

#[test]
fn test_compile_regex() {
    let mut verex: VerEx = VerEx::new();
    verex.add(r"a");

    let regex1 = verex.compile().unwrap();
    assert!(regex1.is_match(r"a"));

    let regex2 = verex.regex().unwrap();
    assert!(regex2.is_match(r"a"));
}

#[test]
fn test_source_and_raw() {
    let mut verex: VerEx = VerEx::from_str(r"a");
    assert_eq!(verex.source(), r"a");
    assert_eq!(verex.raw(), r"a");
}

#[test]
fn test_anything() {
    let mut verex: VerEx = VerEx::new();
    verex.anything();
    assert_eq!(verex.source(), r"(.*)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_anything_but() {
    let mut verex: VerEx = VerEx::new();
    verex.start_of_line()
         .anything_but("foo")
         .end_of_line();
    assert_eq!(verex.source(), r"^([^foo]*)$");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}
