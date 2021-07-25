pub mod parsers {
    use nom::{
        bytes::complete::take_while,
        character::complete::{alphanumeric1, char},
        IResult,
        branch::alt,
        sequence::{preceded, delimited, pair},
        multi::many1,
    };

    #[derive(Debug)]
    pub enum ParsedSNode<'a> {
        Term(&'a str),
        Func(&'a str, Vec<ParsedSNode<'a>>),
    }
    use ParsedSNode::*;

    ////////////////////////////////////////////////////////////////////////////
    // ParseInput custom nom input type with required traits                  //
    // These impl blocks were copied from here changing "self" references     //
    // to self.0                                                              //
    //     https://github.com/Geal/nom/blob/master/doc/custom_input_types.md  //
    ////////////////////////////////////////////////////////////////////////////
    struct ParseInput<'a> (&'a str, &'a str); // (input, constraint)
                                // don't know why semi colon is needed here.


    impl<'a> AsBytes for ParseInput<'a> {
        #[inline(always)]
        fn as_bytes(&self) -> &[u8] {
            (*self.0).as_bytes()
        }
    }

    impl<'a, 'b> Compare<&'b str> for ParseInput<'a> {
      #[inline(always)]
      fn compare(&self, t: &'b str) -> CompareResult {
        self.0.as_bytes().compare(t.as_bytes())
      }

      //FIXME: this version is too simple and does not use the current locale
      #[inline(always)]
      fn compare_no_case(&self, t: &'b str) -> CompareResult {
        let pos = self.0
          .chars()
          .zip(t.chars())
          .position(|(a, b)| a.to_lowercase().ne(b.to_lowercase()));

        match pos {
          Some(_) => CompareResult::Error,
          None => {
            if self.0.len() >= t.len() {
              CompareResult::Ok
            } else {
              CompareResult::Incomplete
            }
          }
        }
      }
    }

    #[cfg(feature = "alloc")]
    impl ExtendInto for ParseInput<'a> {
      type Item = char;
      type Extender = String;

      #[inline]
      fn new_builder(&self) -> String {
        String::new()
      }
      #[inline]
      fn extend_into(&self, acc: &mut String) {
        acc.push_str(self.0);
      }
    }

    impl<'a, 'b> FindSubstring<&'b str> for ParseInput<'a> {
      //returns byte index
      fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.0.find(substr)
      }
    }

    impl<'a> FindToken<u8> for ParseInput<'a> {
      fn find_token(&self, token: u8) -> bool {
        self.0.as_bytes().find_token(token)
      }
    }
    impl<'a> InputIter for ParseInput<'a> {
      type Item = char;
      type Iter = CharIndices<'a>;
      type IterElem = Chars<'a>;
      #[inline]
      fn iter_indices(&self) -> Self::Iter {
        self.0.char_indices()
      }
      #[inline]
      fn iter_elements(&self) -> Self::IterElem {
        self.0.chars()
      }
      fn position<P>(&self, predicate: P) -> Option<usize>
      where
        P: Fn(Self::Item) -> bool,
      {
        for (o, c) in self.0.char_indices() {
          if predicate(c) {
            return Some(o);
          }
        }
        None
      }
      #[inline]
      fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        let mut cnt = 0;
        for (index, _) in self.0.char_indices() {
          if cnt == count {
            return Ok(index);
          }
          cnt += 1;
        }
        if cnt == count {
          return Ok(self.0.len());
        }
        Err(Needed::Unknown)
      }
    }
    impl<'a> InputLength for ParseInput<'a> {
      #[inline]
      fn input_len(&self) -> usize {
        self.0.len()
      }
    }
    impl<'a> InputTake for ParseInput<'a> {
      #[inline]
      fn take(&self, count: usize) -> Self {
        &self[..count]
      }

      // return byte index
      #[inline]
      fn take_split(&self, count: usize) -> (Self, Self) {
        (&self[count..], &self[..count])
      }
    }
    ////////////////////////////////////////////////////////////////////////////

















    pub type ParsedArgs<'a> = Vec<ParsedSNode<'a>>;

    pub fn parse_sexpr(input: &str) -> IResult<&str, ParsedSNode> {
        alt(
            (
                parse_term_node,
                parse_func_node,
            )
        )(input)
    }

    pub fn parse_term_node(input: &str) -> IResult<&str, ParsedSNode> {
        preceded(
            skip_spaces,
            alphanumeric1,
        )(input).map(|(input, output)| (input, Term(output)))
    }

    pub fn parse_func_node(input: &str) -> IResult<&str, ParsedSNode> {
        preceded(
            skip_spaces,
            delimited(
                char('('),
                parse_func_call, 
                preceded(
                    skip_spaces,
                    char(')')
                )
            )
        )(input)
    }

    pub fn parse_func_call(input: &str) -> IResult<&str, ParsedSNode> {
        pair(parse_func_name, parse_func_args)(input).map(|(input, output)|
            (input, Func(output.0, output.1)))
    }

    pub fn parse_func_name(input: &str) -> IResult<&str, &str> {
        preceded(
            skip_spaces,
            alphanumeric1,
        )(input)
    }

    pub fn parse_func_args(input: &str) -> IResult<&str, ParsedArgs> {
        many1(
            alt(
                (
                    parse_term_node,
                    parse_func_node,
                )
            )
        )(input)
    }

    pub fn skip_spaces(input: &str) -> IResult<&str, &str> {
        let chars = " \t\r\n";
        take_while(move |ch| chars.contains(ch))(input)
    }

    #[cfg(test)]
    mod tests {
            use super::*;

            #[test]
            fn test_parse_sexpr() {
                assert_eq!(
                    format!("{:?}", parse_sexpr("(ADD X Y)")), 
                            r#"Ok(("", Func("ADD", [Term("X"), Term("Y")])))"#);
                assert_eq!(
                    format!("{:?}", parse_sexpr("( ADD  X    Y )"
                            )), 
                            r#"Ok(("", Func("ADD", [Term("X"), Term("Y")])))"#);

                assert_eq!(
                    format!("{:?}", parse_sexpr(
                        "( ADD    X (DIV (IF 3 1 3)  2) ( MULT 1 4 1  )  )"
                            )), 
                            r#"Ok(("", Func("ADD", [Term("X"), Func("DIV", [Func("IF", [Term("3"), Term("1"), Term("3")]), Term("2")]), Func("MULT", [Term("1"), Term("4"), Term("1")])])))"#);

                assert_eq!(
                    format!("{:?}", parse_sexpr("( ADD ()   X Y  )")),
                            r#"Err(Error(Error { input: ")   X Y  )", code: AlphaNumeric }))"#);

            }

            #[test]
            fn test_parse_term_node() {
                assert_eq!(
                    format!("{:?}", parse_term_node("X")), 
                            r#"Ok(("", Term("X")))"#);
                assert_eq!(
                    format!("{:?}", parse_term_node("X Y Z")), 
                            r#"Ok((" Y Z", Term("X")))"#);
                assert_eq!(
                    format!("{:?}", parse_term_node("  X123YZ Y92 Z29 ")),
                            r#"Ok((" Y92 Z29 ", Term("X123YZ")))"#);
                assert_eq!(
                    format!("{:?}", parse_term_node(")  X123YZ Y92 Z29 ")),
                            r#"Err(Error(Error { input: ")  X123YZ Y92 Z29 ", code: AlphaNumeric }))"#);
            }

            #[test]
            fn test_parse_func_node() {
                assert_eq!(
                    format!("{:?}", parse_func_node(" ( XYZ 1  2   (AD 3) ) ")),
                            r#"Ok((" ", Func("XYZ", [Term("1"), Term("2"), Func("AD", [Term("3")])])))"#);

                assert_eq!(
                    format!("{:?}", parse_func_node(" ( XYZ 1  2   (AD 3 ) ")),
                            r#"Err(Error(Error { input: "", code: Char }))"#);
            }

            #[test]
            fn test_parse_func_call() {
                assert_eq!(
                    format!("{:?}", parse_func_call("  ABCD123 X1 (Y 1) Z  ")),
                            r#"Ok(("  ", Func("ABCD123", [Term("X1"), Func("Y", [Term("1")]), Term("Z")])))"#);

                assert_eq!(
                    format!("{:?}", parse_func_call("  (ABCD123 X1 (Y 1) Z)  ")),
                            r#"Err(Error(Error { input: "(ABCD123 X1 (Y 1) Z)  ", code: AlphaNumeric }))"#);

            }

            #[test]
            fn test_parse_func_name() {
                assert_eq!(
                    format!("{:?}", parse_func_name("    ABCD123 ")),
                            r#"Ok((" ", "ABCD123"))"#);
                assert_eq!(
                    format!("{:?}", parse_func_name(" (   ABCD123 )")),
                            r#"Err(Error(Error { input: "(   ABCD123 )", code: AlphaNumeric }))"#);
            }

            #[test]
            fn test_parse_func_args() {
                assert_eq!(
                    format!("{:?}", parse_func_args("  X1 (Y 1) Z  ")),
                            r#"Ok(("  ", [Term("X1"), Func("Y", [Term("1")]), Term("Z")]))"#);
                assert_eq!(
                    format!("{:?}", parse_func_args("  (X1 (Y 1) Z   ")),
                            r#"Err(Error(Error { input: "", code: Char }))"#);

                assert_eq!(
                    format!("{:?}", parse_func_args("  ((X1 Y) X) (Y 1) Z   ")),
                            r#"Err(Error(Error { input: "(X1 Y) X) (Y 1) Z   ", code: AlphaNumeric }))"#);
              }

            #[test]
            fn test_skip_spaces() {
                assert_eq!(
                    format!("{:?}", skip_spaces("  \t \r  \n\n AABC DEF ")),
                            r#"Ok(("AABC DEF ", "  \t \r  \n\n "))"#);
            }
    }
}
