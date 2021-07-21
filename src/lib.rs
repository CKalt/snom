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
    pub enum ParsedSNode {
        Term(String),
        Func(String, Vec<ParsedSNode>),
    }
    use ParsedSNode::*;

    pub type ParsedArgs = Vec<ParsedSNode>;

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
        )(input).map(|(input, output)| (input, Term(output.to_string())))
    }

    pub fn parse_func_node(input: &str) -> IResult<&str, ParsedSNode> {
        preceded(
            skip_spaces,
            parse_sub_expr,
        )(input)
    }

    pub fn parse_func_name(input: &str) -> IResult<&str, String> {
        preceded(
            skip_spaces,
            alphanumeric1,
        )(input).map(|(input, output)| (input, output.to_string()))
    }

    pub fn parse_sub_expr(input: &str) -> IResult<&str, ParsedSNode> {
        delimited(
            char('('),
            parse_func_call, 
            preceded(
                skip_spaces,
                char(')')
            )
        )(input)
    }

    pub fn parse_func_call(input: &str) -> IResult<&str, ParsedSNode> {
        pair(parse_func_name, parse_args)(input).map(|(input, output)|
            (input, Func(output.0.to_string(), output.1)))
    }

    pub fn parse_args(input: &str) -> IResult<&str, ParsedArgs> {
        many1(
            alt(
                (
                    parse_term_node,
                    parse_func_node,
                )
            )
        )(input)
    }

    fn skip_spaces(input: &str) -> IResult<&str, &str> {
        let chars = " \t\r\n";
        take_while(move |ch| chars.contains(ch))(input)
    }

    #[cfg(test)]
    mod tests {
            use super::*;

            #[test]
            fn test_parse_term_node() {
                assert_eq!(
                    format!("{:?}", parse_term_node("X")), 
                            r#"Ok(("", Term("X")))"#);
            }

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
            }
    }
}
