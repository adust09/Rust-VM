use assembler::register_parsers::register;

named!(pub operand<CompleteStr, Token>,
alt!(
    integer_operand |
    register
    )
);
