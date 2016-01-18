use Verex::Verex;

const A_Verex_STRING: &'static str = r"(?:a)";

#[test]
fn test_constructors() {
    let Verex1: Verex = Verex::new();
    assert_eq!(Verex1.source(), r"(?:)");

    let Verex2: Verex = Verex::from_string(r"a".to_string());
    assert_eq!(Verex2.source(), A_Verex_STRING);

    let Verex3: Verex = Verex::from_str(r"a");
    assert_eq!(Verex3.source(), A_Verex_STRING);
}

#[test]
fn test_add() {
    let mut Verex: Verex = Verex::new();
    Verex.add(r"a");
    Verex.update_source_with_modifiers();
    assert_eq!(Verex.source(), A_Verex_STRING);
}

#[test]
fn test_compile_regex() {
    let mut Verex: Verex = Verex::new();
    Verex.find(r"a");

    let regex1 = Verex.compile().unwrap();
    assert!(regex1.is_match(r"a"));

    let regex2 = Verex.regex().unwrap();
    assert!(regex2.is_match(r"a"));
}

#[test]
fn test_i_modifier() {
    let mut Verex = Verex::from_str(r"a");
    Verex.with_any_case(true);
    assert_eq!(Verex.source(), r"(?i:a)");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"A"));
    assert!(!regex.is_match(r"b"));
}

#[test]
fn test_m_modifier() {
    let Verex = Verex::new()
                   .start_of_line()
                   .find(r"a")
                   .end_of_line()
                   .search_one_line(false)
                   .clone();
    assert_eq!(Verex.source(), r"(?m:^(?:a)$)");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"aa"));
    assert!(regex.is_match("a\n"));
    assert_eq!(regex.find_iter("a\na").count(), 2);
}

#[test]
fn test_source_and_raw_and_value() {
    let Verex: Verex = Verex::from_str(r"a");
    assert_eq!(Verex.source(), A_Verex_STRING);
    assert_eq!(Verex.raw(), A_Verex_STRING);
    assert_eq!(Verex.value(), A_Verex_STRING);
}

#[test]
fn test_any_and_any_of() {
    let mut Verex1: Verex = Verex::new();
    Verex1.any(r"ab");

    let regex1 = Verex1.compile().unwrap();
    assert!(regex1.is_match(r"a"));
    assert!(regex1.is_match(r"b"));
    assert!(!regex1.is_match(r"c"));

    let mut Verex2: Verex = Verex::new();
    Verex2.any_of(r"ab");

    let regex2 = Verex2.compile().unwrap();
    assert!(regex2.is_match(r"a"));
    assert!(regex2.is_match(r"b"));
    assert!(!regex2.is_match(r"c"));
}

#[test]
fn test_anything() {
    let mut Verex: Verex = Verex::new();
    Verex.anything();
    assert_eq!(Verex.source(), r"(?:(.*))");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_anything_but() {
    let mut Verex: Verex = Verex::new();
    Verex.start_of_line()
         .anything_but("foo")
         .end_of_line();
    assert_eq!(Verex.source(), r"(?:^(?:[^foo]*)$)");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_digit() {
    let Verex = Verex::new().digit().clone();
    assert_eq!(Verex.source(), r"(?:\d)");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r"0"));
    assert!(regex.is_match(r"1"));
    assert!(regex.is_match(r"3"));
    assert!(regex.is_match(r"9"));
    assert!(!regex.is_match(r"a"));
    assert!(!regex.is_match(r" "));
    assert!(!regex.is_match(r"?"));
}

#[test]
fn test_find_and_then() {
    let mut Verex: Verex = Verex::new();
    Verex.find("foo");
    assert_eq!(Verex.source(), r"(?:(?:foo))");

    let regex = Verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(regex.is_match(r"foo"));
    assert!(regex.is_match(r"foofoo"));
    assert!(regex.is_match(r"barfoo"));

    // same as find
    let mut Verex2: Verex = Verex::new();
    Verex2.then("foo");
    assert_eq!(Verex2.source(), r"(?:(?:foo))");

    let regex2 = Verex2.compile().unwrap();
    assert!(!regex2.is_match(r"bar"));
    assert!(regex2.is_match(r"foo"));
    assert!(regex2.is_match(r"foofoo"));
    assert!(regex2.is_match(r"barfoo"));
}

#[test]
fn test_find_chained() {
    let mut Verex: Verex = Verex::new();
    Verex.find("foo")
         .then("bar");
    assert_eq!(Verex.source(), r"(?:(?:foo)(?:bar))");

    let regex = Verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"barfoo"));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_maybe() {
    let mut Verex: Verex = Verex::new();
    Verex.start_of_line()
         .maybe(r"a")
         .end_of_line();
    assert_eq!(Verex.source(), r"(?:^(?:a)?$)");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"foo"));
}

#[test]
fn test_or_and_or_find() {
    let mut Verex1 = Verex::new();
    Verex1.find(r"a")
          .or()
          .find(r"b");
    assert_eq!(Verex1.source(), r"(?:(?:a)|(?:b))");

    let regex1 = Verex1.compile().unwrap();
    assert!(regex1.is_match(r"a"));
    assert!(regex1.is_match(r"b"));
    assert!(!regex1.is_match(r"z"));

    let mut Verex2 = Verex::new();
    Verex2.find(r"a")
          .or_find(r"b");
    assert_eq!(Verex2.source(), r"(?:(?:a)|(?:b))");

    let regex2 = Verex2.compile().unwrap();
    assert!(regex2.is_match(r"a"));
    assert!(regex2.is_match(r"b"));
    assert!(!regex2.is_match(r"z"));
}

#[test]
fn test_range() {
    let mut Verex = Verex::new();
    Verex.range(vec![('a', 'z')]);
    assert_eq!(Verex.source(), r"(?:[a-z])");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"b"));
    assert!(regex.is_match(r"h"));
    assert!(regex.is_match(r"u"));
    assert!(regex.is_match(r"z"));
    assert!(!regex.is_match(r"A"));
    assert!(!regex.is_match(r"Z"));
}

#[test]
fn test_replace() {
    let Verex = Verex::from_str(r"r");
    let replaced = Verex.replace(r"foobar", r"z").unwrap();
    assert_eq!(replaced, r"foobaz");
}

#[test]
fn test_something() {
    let mut Verex: Verex = Verex::new();
    Verex.something();
    assert_eq!(Verex.source(), r"(?:(.+))");

    let regex = Verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_someting_but() {
    let mut Verex: Verex = Verex::new();
    Verex.start_of_line()
         .something_but("foo")
         .end_of_line();
    assert_eq!(Verex.source(), r"(?:^(?:[^foo]+)$)");

    let regex = Verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_word() {
    let mut Verex = Verex::new();
    Verex.word();
    assert_eq!(Verex.source(), r"(?:(?:\w+))");

    let regex = Verex.compile().unwrap();
    assert!(regex.is_match(r"word"));
    assert!(regex.is_match(r"w0rd"));
    assert!(!regex.is_match(r"./"));
}

// test the standalone functions
