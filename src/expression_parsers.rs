use nom::types::CompleteStr;

use operator_parsers::*;
use term_parsers::term;
use tokens::Token;

named!(pub expression<CompleteStr,Token>,
    do_parse!(
        left: term >>
        right: many0!(
            tuple!(
                alt!(
                    addition_operator |
                    subtraction_operator
                ),
                term
            )
        ) >>
        (
            {
                Token::Expression{left: Box::new(left), right: right}
            }
        )
    )
);

#[test]
fn test_parse_expression() {
    let result = expression(CompleteStr("3*4"));
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_parse_nested_expression() {
    let result = expression(CompleteStr("(3*4)+1"));
    assert_eq!(result.is_ok(), true);
}
