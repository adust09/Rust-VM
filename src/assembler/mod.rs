use crate::instruction::Opcode;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new()
        }
    }
}

pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
    match program(CompleteStr(raw)) {
        Ok((_remainder, program)) => {
            self.process_first_phase(&program);
            Some(self.process_second_phase(&program))
        },
        Err(e) => {
            println!("There was an error assembling the code: {:?}", e);
            None
        }
    }
}