named!(directive_declaration<CompleteStr,Token>,
do_parse!(
    tag!(".")>>
    name: alpha1 >>
    (
        Token::Directive{name: name.to_string()}
    )
)
);

named!(directive_combined<CompleteStr,AssemblerInstruction>,
    ws!(
        do_parse!(
            tag!("." ) >>
            name: directive_declaration >>
            o1: opt!(operand) >>
            o2: opt!(operand) >>
            o3: opt!(operand) >>
            (
                AssemblerINstruction{
                    opecode: None,
                    directive: Some(name),
                    label: None,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3,
                }
            )
        )
    )
);

//will try to parse out any of the Directive forms
named!(pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            directive_combined
        ) >>
        (
            ins
        )
    )
);