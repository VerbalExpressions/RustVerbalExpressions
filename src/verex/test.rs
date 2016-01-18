use verex::VerEx;

const A_VEREX_STRING: &'static str = r"(?:a)";

#[test]
fn test_constructors() {
    let verex1: VerEx = VerEx::new();
    assert_eq!(verex1.source(), r"(?:)");

    let verex2: VerEx = VerEx::from_string(r"a".to_string());
    assert_eq!(verex2.source(), A_VEREX_STRING);

    let verex3: VerEx = VerEx::from_str(r"a");
    assert_eq!(verex3.source(), A_VEREX_STRING);
}

#[test]
fn test_add() {
    let mut verex: VerEx = VerEx::new();
    verex.add(r"a");
    verex.update_source_with_modifiers();
    assert_eq!(verex.source(), A_VEREX_STRING);
}

#[test]
fn test_compile_regex() {
    let mut verex: VerEx = VerEx::new();
    verex.find(r"a");

    let regex1 = verex.compile().unwrap();
    assert!(regex1.is_match(r"a"));

    let regex2 = verex.regex().unwrap();
    assert!(regex2.is_match(r"a"));
}

#[test]
fn test_i_modifier() {
    let mut verex = VerEx::from_str(r"a");
    verex.with_any_case(true);
    assert_eq!(verex.source(), r"(?i:a)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(regex.is_match(r"A"));
    assert!(!regex.is_match(r"b"));
}

#[test]
fn test_m_modifier() {
    let verex = VerEx::new()
                   .start_of_line()
                   .find(r"a")
                   .end_of_line()
                   .search_one_line(false)
                   .clone();
    assert_eq!(verex.source(), r"(?m:^(?:a)$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"aa"));
    assert!(regex.is_match("a\n"));
    assert_eq!(regex.find_iter("a\na").count(), 2);
}

#[test]
fn test_source_and_raw_and_value() {
    let verex: VerEx = VerEx::from_str(r"a");
    assert_eq!(verex.source(), A_VEREX_STRING);
    assert_eq!(verex.raw(), A_VEREX_STRING);
    assert_eq!(verex.value(), A_VEREX_STRING);
}

#[test]
fn test_any_and_any_of() {
    let mut verex1: VerEx = VerEx::new();
    verex1.any(r"ab");

    let regex1 = verex1.compile().unwrap();
    assert!(regex1.is_match(r"a"));
    assert!(regex1.is_match(r"b"));
    assert!(!regex1.is_match(r"c"));

    let mut verex2: VerEx = VerEx::new();
    verex2.any_of(r"ab");

    let regex2 = verex2.compile().unwrap();
    assert!(regex2.is_match(r"a"));
    assert!(regex2.is_match(r"b"));
    assert!(!regex2.is_match(r"c"));
}

#[test]
fn test_anything() {
    let mut verex: VerEx = VerEx::new();
    verex.anything();
    assert_eq!(verex.source(), r"(?:(.*))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_anything_but() {
    let mut verex: VerEx = VerEx::new();
    verex.start_of_line()
         .anything_but("foo")
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:[^foo]*)$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_digit() {
    let verex = VerEx::new().digit().clone();
    assert_eq!(verex.source(), r"(?:\d)");

    let regex = verex.compile().unwrap();
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
    let mut verex: VerEx = VerEx::new();
    verex.find("foo");
    assert_eq!(verex.source(), r"(?:(?:foo))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(regex.is_match(r"foo"));
    assert!(regex.is_match(r"foofoo"));
    assert!(regex.is_match(r"barfoo"));

    // same as find
    let mut verex2: VerEx = VerEx::new();
    verex2.then("foo");
    assert_eq!(verex2.source(), r"(?:(?:foo))");

    let regex2 = verex2.compile().unwrap();
    assert!(!regex2.is_match(r"bar"));
    assert!(regex2.is_match(r"foo"));
    assert!(regex2.is_match(r"foofoo"));
    assert!(regex2.is_match(r"barfoo"));
}

#[test]
fn test_find_chained() {
    let mut verex: VerEx = VerEx::new();
    verex.find("foo")
         .then("bar");
    assert_eq!(verex.source(), r"(?:(?:foo)(?:bar))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"barfoo"));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_maybe() {
    let mut verex: VerEx = VerEx::new();
    verex.start_of_line()
         .maybe(r"a")
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:a)?$)");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"a"));
    assert!(!regex.is_match(r"foo"));
}

#[test]
fn test_or_and_or_find() {
    let mut verex1 = VerEx::new();
    verex1.find(r"a")
          .or()
          .find(r"b");
    assert_eq!(verex1.source(), r"(?:(?:a)|(?:b))");

    let regex1 = verex1.compile().unwrap();
    assert!(regex1.is_match(r"a"));
    assert!(regex1.is_match(r"b"));
    assert!(!regex1.is_match(r"z"));

    let mut verex2 = VerEx::new();
    verex2.find(r"a")
          .or_find(r"b");
    assert_eq!(verex2.source(), r"(?:(?:a)|(?:b))");

    let regex2 = verex2.compile().unwrap();
    assert!(regex2.is_match(r"a"));
    assert!(regex2.is_match(r"b"));
    assert!(!regex2.is_match(r"z"));
}

#[test]
fn test_range() {
    let mut verex = VerEx::new();
    verex.range(vec![('a', 'z')]);
    assert_eq!(verex.source(), r"(?:[a-z])");

    let regex = verex.compile().unwrap();
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
    let mut verex = VerEx::from_str(r"foobar");
    verex.replace(r"r", r"z");
    assert_eq!(verex.source(), r"(?:foobaz)");
}

#[test]
fn test_something() {
    let mut verex: VerEx = VerEx::new();
    verex.something();
    assert_eq!(verex.source(), r"(?:(.+))");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_someting_but() {
    let mut verex: VerEx = VerEx::new();
    verex.start_of_line()
         .something_but("foo")
         .end_of_line();
    assert_eq!(verex.source(), r"(?:^(?:[^foo]+)$)");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_word() {
    let mut verex = VerEx::new();
    verex.word();
    assert_eq!(verex.source(), r"(?:(?:\w+))");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r"word"));
    assert!(regex.is_match(r"w0rd"));
    assert!(!regex.is_match(r"./"));
}

// test the standalone functions
