use verex::VerEx;

#[test]
fn test_constructors() {
    let verex1: VerEx = VerEx::new();
    assert_eq!(verex1.source(), r"");

    let verex2: VerEx = VerEx::from_string(r"a".to_string());
    assert_eq!(verex2.source(), r"a");

    let verex3: VerEx = VerEx::from_str(r"a");
    assert_eq!(verex3.source(), r"a");
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
    let verex: VerEx = VerEx::from_str(r"a");
    assert_eq!(verex.source(), r"a");
    assert_eq!(verex.raw(), r"a");
}

#[test]
fn test_anything() {
    let mut verex: VerEx = VerEx::new();
    verex.anything();
    assert_eq!(verex.source(), r"(.*)");

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
    assert_eq!(verex.source(), r"^([^foo]*)$");

    let regex = verex.compile().unwrap();
    assert!(regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}

#[test]
fn test_find_and_then() {
    let mut verex: VerEx = VerEx::new();
    verex.find("foo");
    assert_eq!(verex.source(), r"(foo)");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(regex.is_match(r"foo"));
    assert!(regex.is_match(r"foofoo"));
    assert!(regex.is_match(r"barfoo"));

    // same as find
    let mut verex2: VerEx = VerEx::new();
    verex2.then("foo");
    assert_eq!(verex2.source(), r"(foo)");

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
    assert_eq!(verex.source(), r"(foo)(bar)");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"barfoo"));
    assert!(regex.is_match(r"foobar"));
}

#[test]
fn test_something() {
    let mut verex: VerEx = VerEx::new();
    verex.something();
    assert_eq!(verex.source(), r"(.+)");

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
    assert_eq!(verex.source(), r"^([^foo]+)$");

    let regex = verex.compile().unwrap();
    assert!(!regex.is_match(r""));
    assert!(regex.is_match(r"bar"));
    assert!(!regex.is_match(r"foo"));
    assert!(!regex.is_match(r"foofoo"));
    assert!(!regex.is_match(r"barfoo"));
}
