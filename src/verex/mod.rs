extern crate regex;

use self::regex::Regex;
use self::regex::Error;

#[cfg(test)] pub mod test;

pub struct VerEx {
    string: String
}

impl VerEx {
    // constructors
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

    // fundamental methods
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

    /// Any character zero or more times
    pub fn anything(&mut self) -> &mut VerEx {
        self.add(r"(.*)")
    }

    /// Any character zero or more times except the provided characters
    pub fn anything_but(&mut self, value: &str) -> &mut VerEx {
        let mut string = r"([^".to_string();
        string.push_str(value);
        string.push_str(r"]*)");
        self.add(string.as_ref())
    }

    pub fn end_of_line(&mut self) -> &mut VerEx {
        self.add(r"$")
    }

    /// Find a specific string
    pub fn find(&mut self, value: &str) -> &mut VerEx {
        let mut string = r"(".to_string();
        string.push_str(value);
        string.push_str(r")");
        self.add(string.as_ref())
    }

    /// Any character at least one time
    pub fn something(&mut self) -> &mut VerEx {
        self.add(r"(.+)")
    }

    /// Any character at least one time except for these characters
    pub fn something_but(&mut self, value: &str) -> &mut VerEx {
        let mut string = r"([^".to_string();
        string.push_str(value);
        string.push_str(r"]+)");
        self.add(string.as_ref())
    }

    pub fn start_of_line(&mut self) -> &mut VerEx {
        self.add(r"^")
    }

    /// To use find "in the sentence" and make the chaining flow better
    pub fn then(&mut self, value: &str) -> &mut VerEx {
        self.find(value)
    }
}
