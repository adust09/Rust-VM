use nom::tpes::CompleteStr;

use assembler::instruction_parsers::{instruction_one, Assemblerinstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instruction: Vec<AssemblerInstruction>,
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut progoram = vec![];
        for instruction in &self.instruction {
            progoram.append(&mut instruction.to_bytes());
        }
        program
    }
}

#[test]
fn test_parse_program() {
    let result = program(CompleteStr("load $0 #100\n"));
    assert_eq!(result.is_ok(), true);
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, CompleteStr(""));
    assert_eq!(1, p.instructions.len());
    // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
}

#[test]
fn test_program_to_bytes() {
    let result = progaram(CompleteStr("load $0 #100\n"));
    assert_eq!(result.is_ok(), true);
    let (_, program) = result.unwrap();
    let bytecode = program.to_bytes();
    assert_eq!(bytescode.len(), 4);
    println!("{:?}", bytecode);
}
