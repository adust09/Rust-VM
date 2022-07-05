use crate::assembler::register_parsers::register;
use crate::assembler::label_parsers::label_usage;

use crate::assembler::{Token, Assembler};
use crate::instruction::Opcode;
use nom::digit;
use nom::types::CompleteStr;
use nom::*;

/// Parser for integer numbers, which we preface with `#` in our assembly language:
/// #100
named!(integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                // Token::Number{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

named!(pub operand<CompleteStr,Token>,
    alt!(
        integer_operand |
        register
    )
);

named!(irstring<CompleteStr,Token>,
do_parse!(
    tag!("") >>
    content: take_util!("'") >>
    tag!("'") >>
    (
        Token::IrString{name:content.to_string()}
    )
));

named!(pub operand<CompleteStr,Token>,
alt!(
    integer_operand |
    label_usage |
    register |
    irstring
));

#[test]
fn test_parse_integer_operand() {
    // Test a valid integer operand
    let result = integer_operand(CompleteStr("#10"));
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::IntegerOperand { value: 10 });

    // Test an invalid one (missing the #)
    let result = integer_operand(CompleteStr("10"));
    assert_eq!(result.is_ok(), false);
}

#[test]
fn test_parse_string_operand() {
    let result = irstring(CompleteStr("'This is a test'"));
    assert_eq!(result.is_ok(), true);
}

    #[test]
    fn test_string_directive() {
        let result = directive_combined(CompleteStr("test: .asciiz 'Hello'"));
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.unwrap();

        // Yes, this is the what the result should be
        let correct_instruction = AssemblerInstruction {
            opcode: None,
            label: Some(Token::LabelDeclaration { name: "test".to_string() }),
            directive: Some(Token::Directive { name: "asciiz".to_string() }),
            operand1: Some(Token::IrString { name: "Hello".to_string() }),
            operand2: None,
            operand3: None,
        };

        assert_eq!(directive, correct_instruction);
    }

#[test]
fn test_complete_program() {
    let test_program = CompleteStr(".data\nhello: .asciiz 'Hello everyone!'\n.code\nhlt");
    let result = program(test_program);
    assert_eq!(result.is_ok(), true);
}