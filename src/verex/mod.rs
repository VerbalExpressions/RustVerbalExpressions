pub use regex::Regex;
use regex::Error;

#[cfg(test)] pub mod test;

bitflags! {
    flags Modifiers: u8 {
        const MULTI_LINE        = 0b00000001,
        const CASE_INSENSITIVE  = 0b00000010,
    }
}

/// The struct used for keeping track of the modifiers
impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers::empty()
    }
}

/// The enum used to have common functions for different Types
pub enum Expression<'e> {
    /// The variant for handing over a String reference
    String(&'e str),
    /// The variant for handing over a Verex reference
    Verex(&'e Verex),
    /// The variant for handing over a Regex reference
    Regex(&'e Regex)
}

macro_rules! match_expr {
    ( $e:expr, $this:expr, $method:ident ) => {
        {
            match $e {
                Expression::String(x) => { $this.$method(x) },
                Expression::Verex(x) => { $this.$method(x.source()) },
                Expression::Regex(x) => { $this.$method(x.as_str()) },
            }
        }
    }
}

const ESCAPE_PAIRS: [(&'static str, &'static str); 14] = [
    (r"(?P<back_slash>\\)", "\\$back_slash"),
    (r"(?P<open_group>\()", "\\$open_group"),
    (r"(?P<close_group>\))", "\\$close_group"),
    (r"(?P<open_class>\[)", "\\$open_class"),
    (r"(?P<close_class>\])", "\\$close_class"),
    (r"(?P<open_quantifier>\{)", "\\$open_quantifier"),
    (r"(?P<close_quantifier>\})", "\\$close_quantifier"),
    (r"(?P<dot>\.)", "\\$dot"),
    (r"(?P<plus>\+)", "\\$plus"),
    (r"(?P<star>\*)", "\\$star"),
    (r"(?P<question_mark>\?)", "\\$question_mark"),
    (r"(?P<circumflex>\^)", "\\$circumflex"),
    (r"(?P<dollar>\$)", "\\$dollar"),
    (r"(?P<pipe>\|)", "\\$pipe"),
];

fn escape(string: &str) -> String {
    let mut result = string.to_string();
    for pair in ESCAPE_PAIRS.into_iter() {
        let regex = Regex::new(pair.0).unwrap();
        result = regex.replace_all(result.as_ref(), pair.1);
    }
    result
}

/// The struct used for building verbal expression objects
#[derive(Debug, Clone)]
pub struct Verex {
    string: String,
    modifiers: Modifiers,
    source: String
}

impl Verex {
    // constructors
    /// Standard Constructor
    pub fn new() -> Verex {
        Verex::from_string(String::new())
    }

    /// Create a `Verex` object from a `String`
    pub fn from_string(string: String) -> Verex {
        let mut verex = Verex {
            string: string,
            modifiers: Modifiers::new(),
            source: String::new()
        };
        verex.update_source_with_modifiers();
        verex
    }

    /// Create a `Verex` object from a `&str`
    pub fn from_str(string: &str) -> Verex {
        Verex::from_string(string.to_string())
    }

    // --------------------------------------------------
    // fundamental methods
    /// Add a string to the regex string in the `Verex` and return self
    fn add(&mut self, value: &str) -> &mut Verex {
        self.string.push_str(value);
        self
    }

    /// Update the source string from the (presumably changed) builder string
    fn update_source_with_modifiers(&mut self) -> &mut Verex {
        self.source.clear();
        self.source.push_str(r"(?");
        if self.modifiers.contains(CASE_INSENSITIVE) {
            self.source.push('i');
        }
        if self.modifiers.contains(MULTI_LINE) {
            self.source.push('m');
        }
        self.source.push(':');
        self.source.push_str(self.string.as_ref());
        self.source.push(')');
        self
    }

    /// Compile the `Verex` to a `Regex` and return the result
    pub fn compile(& self) -> Result<Regex, Error> {
        Regex::new(self.source.as_ref())
    }

    /// Return the raw regex string contained in the `Verex`
    pub fn raw(& self) -> &str {
        self.source()
    }

    /// Compile the `Verex` to a `Regex` and return the result
    pub fn regex(& self) -> Result<Regex, Error> {
        self.compile()
    }

    /// Return the raw regex string contained in the `Verex`
    pub fn source(& self) -> &str {
        self.source.as_ref()
    }

    /// Return the raw regex string contained in the `Verex`
    pub fn value(& self) -> &str {
        self.source()
    }

    /// Open a character class
    fn open_class(&mut self) -> &mut Verex {
        self.add(r"[")
    }

    /// Close a character class
    fn close_class(&mut self) -> &mut Verex {
        self.add(r"]")
    }

    /// Open a non-capturing group
    fn open_group(&mut self) -> &mut Verex {
        self.add(r"(?:")
    }

    /// Open a capturing group
    fn open_capturing_group(&mut self) -> &mut Verex {
        self.add(r"(")
    }

    /// Close a capturing or non-capturing group
    fn close_group(&mut self) -> &mut Verex {
        self.add(r")")
    }

    /// Open a quantifier
    fn open_quantifier(&mut self) -> &mut Verex {
        self.add(r"{")
    }

    /// Close a quantifier
    fn close_quantifier(&mut self) -> &mut Verex {
        self.add(r"}")
    }

    // --------------------------------------------------

    /// Any of the given characters
    pub fn any(&mut self, chars: &str) -> &mut Verex {
        self.open_class()
            .add(escape(chars).as_ref())
            .close_class();
        self.update_source_with_modifiers()
    }

    /// See `any()`
    pub fn any_of(&mut self, chars: &str) -> &mut Verex {
        self.any(chars)
    }

    /// Any character zero or more times
    pub fn anything(&mut self) -> &mut Verex {
        self.add(r"(.*)");
        self.update_source_with_modifiers()
    }

    /// Any character zero or more times except the provided characters
    pub fn anything_but(&mut self, chars: &str) -> &mut Verex {
        self.open_group()
            .open_class()
            .add(r"^")
            .add(escape(chars).as_ref())
            .close_class()
            .add(r"*")
            .close_group();
        self.update_source_with_modifiers()
    }

    /// A line break!
    pub fn br(&mut self) -> &mut Verex {
        self.line_break()
    }

    /// Find a specific value and capture it
    fn capture_value(&mut self, value: &str) -> &mut Verex {
        self.open_capturing_group()
            .add(value)
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Find a specific string and capture it (will be escaped)
    pub fn capture(&mut self, value: &str) -> &mut Verex {
        self.capture_value(escape(value).as_ref())
    }

    /// Find a specific string and capture it
    pub fn capture_expr(&mut self, expr: Expression) -> &mut Verex {
        match_expr!(expr, self, capture_value)
    }

    /// Add the token for matching digits
    pub fn digit(&mut self) -> &mut Verex {
        self.add(r"\d");
        self.update_source_with_modifiers()
    }

    /// Add a token for matching the end of a line
    pub fn end_of_line(&mut self) -> &mut Verex {
        self.add(r"$");
        self.update_source_with_modifiers()
    }

    /// Find a value
    fn find_value(&mut self, value: &str) -> &mut Verex {
        self.open_group()
            .add(value)
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Find a specific string that will be escaped
    pub fn find(&mut self, value: &str) -> &mut Verex {
        self.find_value(escape(value).as_ref())
    }

    /// Find an expression (does not get escaped)
    pub fn find_expr(&mut self, expr: Expression) -> &mut Verex {
        match_expr!(expr, self, find_value)
    }

    /// A line break!
    pub fn line_break(&mut self) -> &mut Verex {
        self.open_group()
            .add(r"\n")
            .or_find_expr(Expression::String(r"\r\n"))
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Any string either one or zero times
    fn maybe_value(&mut self, value: &str) -> &mut Verex {
        self.open_group()
            .add(value)
            .close_group()
            .add(r"?");
        self.update_source_with_modifiers()
    }

    /// Any string either one or zero times
    pub fn maybe(&mut self, value: &str) -> &mut Verex {
        self.maybe_value(escape(value).as_ref())
    }

    /// Any string either one or zero times
    pub fn maybe_expr(&mut self, expr: Expression) -> &mut Verex {
        match_expr!(expr, self, maybe_value)
    }

    /// Either match the sub-expression before or after this
    pub fn or(&mut self) -> &mut Verex {
        self.add(r"|");
        self.update_source_with_modifiers()
    }

    /// Either match the sub-expression before or the provided value
    pub fn or_find(&mut self, value: &str) -> &mut Verex {
        self.or()
            .find(value)
    }

    /// Either match the sub-expression before or the provided sub-expression
    pub fn or_find_expr(&mut self, expr: Expression) -> &mut Verex {
        self.or()
            .find_expr(expr)
    }

    /// A range of characters e.g. [A-Z]
    /// Usage example: verex.range(vec![('a', 'z'),('A', 'Z')])
    pub fn range(&mut self, range: Vec<(char, char)>) -> &mut Verex {
        let mut string = r"[".to_string();
        for tuple in range {
            let from = tuple.0;
            let to = tuple.1;
            string.push(from);
            string.push('-');
            string.push(to);
        }
        string.push(']');
        self.add(string.as_ref());
        self.update_source_with_modifiers()
    }

    /// Repeat the previous item n times
    pub fn repeat_n(&mut self, n: u32) -> &mut Verex {
        self.open_quantifier()
            .add(n.to_string().as_ref())
            .close_quantifier();
        self.update_source_with_modifiers()
    }

    /// Repeat the previous item n to m times
    pub fn repeat_n_to_m(&mut self, n: u32, m: u32) -> &mut Verex {
        self.open_quantifier()
            .add(n.to_string().as_ref())
            .add(r",")
            .add(m.to_string().as_ref())
            .close_quantifier();
        self.update_source_with_modifiers()
    }

    /// Repeat the previous item once or more times
    pub fn repeat_once_or_more(&mut self) -> &mut Verex {
        self.add(r"+");
        self.update_source_with_modifiers()
    }

    /// Repeat the previous item n times
    pub fn repeat_previous(&mut self, n: u32) -> &mut Verex {
        self.repeat_n(n)
    }

    /// Repeat the previous item zero or more times
    pub fn repeat_zero_or_more(&mut self) -> &mut Verex {
        self.add(r"*");
        self.update_source_with_modifiers()
    }

    /// Replace a substring
    pub fn replace(& self, text: &str, replacement: &str) -> Result<String, Error> {
        let regex = try!(self.compile());
        Ok(regex.replace(text, replacement))
    }

    /// Toggle whether ^ and $ match line start and end or string start and end
    pub fn search_one_line(&mut self, enable: bool) -> &mut Verex {
        if enable {
            self.modifiers.remove(MULTI_LINE);
        }
        else {
            self.modifiers.insert(MULTI_LINE);
        }
        self.update_source_with_modifiers()
    }

    /// Any character at least one time
    pub fn something(&mut self) -> &mut Verex {
        self.add(r"(.+)");
        self.update_source_with_modifiers()
    }

    /// Any character at least one time except for these characters
    pub fn something_but(&mut self, chars: &str) -> &mut Verex {
        self.open_group()
            .open_class()
            .add(r"^")
            .add(escape(chars).as_ref())
            .close_class()
            .add(r"+")
            .close_group();
        self.update_source_with_modifiers()
    }

    /// Add a token for the start of a line
    pub fn start_of_line(&mut self) -> &mut Verex {
        self.add(r"^");
        self.update_source_with_modifiers()
    }

    /// Add a token for a tab
    pub fn tab(&mut self) -> &mut Verex {
        self.add(r"\t");
        self.update_source_with_modifiers()
    }

    /// To use find "in the sentence" and make the chaining flow better
    pub fn then(&mut self, value: &str) -> &mut Verex {
        self.find(value)
    }

    /// Toggle whether to match case-sensitively or not
    pub fn with_any_case(&mut self, enable: bool) -> &mut Verex {
        if enable {
            self.modifiers.insert(CASE_INSENSITIVE);
        }
        else {
            self.modifiers.remove(CASE_INSENSITIVE);
        }
        self.update_source_with_modifiers()
    }

    /// Any alphanumeric characters
    pub fn word(&mut self) -> &mut Verex {
        self.find_expr(Expression::String(r"\w+"))
    }
}

use std::fmt;

impl fmt::Display for Verex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Eq for Verex {}

use std::str::FromStr;
pub enum Void {}

impl FromStr for Verex {
    type Err = Void;

    fn from_str(s: &str) -> Result<Verex, Void> {
        Ok(Verex::from_str(s))
    }
}

/// Equality comparison is based on the original string. It is possible that different verbal expressions have the same matching behavior, but are still compared unequal.
impl PartialEq for Verex {
    fn eq(&self, other: &Verex) -> bool {
        self.string == other.string
    }
}
