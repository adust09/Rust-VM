use nom::types::CompleteStr;

use crate::instruction::Opcode;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;
pub mod program_parsers;

pub mod directive_parsers;
pub mod instruction_parsers;
use crate::assembler::program_parsers::{program, Program};
use crate::assembler::assembler_errors::AssemblerError;


#[derive(Debug, Default)]
pub struct Assembler {
    /// Tracks which phase the assember is in
    phase: AssemblerPhase,
    /// Symbol table for constants and variables
    pub symbols: SymbolTable,
    /// The read-only data section constants are put in
    pub ro: Vec<u8>,
    /// The compiled bytecode generated from the assembly instructions
    pub bytecode: Vec<u8>,
    /// Tracks the current offset of the read-only section
    ro_offset: u32,
    /// A list of all the sections we've seen in the code
    sections: Vec<AssemblerSection>,
    /// The current section the assembler is in
    current_section: Option<AssemblerSection>,
    /// The current instruction the assembler is converting to bytecode
    current_instruction: u32,
    /// Any errors we find along the way. At the end, we'll present them to the user.
    errors: Vec<AssemblerError>
}

pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
    // Runs the raw program through our `nom` parser
    match program(CompleteStr(raw)) {
        // If there were no parsing errors, we now have a `Vec<AssemblyInstructions>` to process.
        // `remainder` _should_ be "".
        // TODO: Add a check for `remainder`, make sure it is "".
        Ok((_remainder, program)) => {
            // First get the header so we can smush it into the bytecode later
            let mut assembled_program = self.write_pie_header();

            // Start processing the AssembledInstructions. This is the first pass of our two-pass assembler.
            // We pass a read-only reference down to another function.
            self.process_first_phase(&program);

            // If we accumulated any errors in the first pass, return them and don't try to do the second pass
            if !self.errors.is_empty() {
                // TODO: Can we avoid a clone here?
                return Err(self.errors.clone());
            };

            // Make sure that we have at least one data section and one code section
            if self.sections.len() != 2 {
                // TODO: Detail out which one(s) are missing
                println!("Did not find at least two sections.");
                self.errors.push(AssemblerError::InsufficientSections);
                // TODO: Can we avoid a clone here?
                return Err(self.errors.clone());
            }
            // Run the second pass, which translates opcodes and associated operands into the bytecode
            let mut body = self.process_second_phase(&program);

            // Merge the header with the populated body vector
            assembled_program.append(&mut body);
            Ok(assembled_program)
        },
        // If there were parsing errors, bad syntax, etc, this arm is run
        Err(e) => {
            println!("There was an error parsing the code: {:?}", e);
            Err(vec![AssemblerError::ParseError{ error: e.to_string() }])
        }
    }
}

/// Runs the first pass of the two-pass assembling process. It looks for labels and puts them in the symbol table
fn process_first_phase(&mut self, p: &Program) {
    // Iterate over every instruction, even though in the first phase we care about labels and directives but nothing else
    for i in &p.instructions {
        if i.is_label() {
            // TODO: Factor this out into another function? Put it in `process_label_declaration`?
            if self.current_section.is_some() {
                // If we have hit a segment header already (e.g., `.code`) then we are ok
                self.process_label_declaration(&i);
            } else {
                // If we have *not* hit a segment header yet, then we have a label outside of a segment, which is not allowed
                self.errors.push(AssemblerError::NoSegmentDeclarationFound{instruction: self.current_instruction});
            }
        }

        if i.is_directive() {
            self.process_directive(i);
        }
        // This is used to keep track of which instruction we hit an error on
        // TODO: Do we really need to track this?
        self.current_instruction += 1;
    }
    // Once we're done with this function, set the phase to second
    self.phase = AssemblerPhase::Second;
}

/// Handles the declaration of a label such as:
/// hello: .asciiz 'Hello'
fn process_label_declaration(&mut self, i: &AssemblerInstruction) {
    // Check if the label is None or String
    let name = match i.get_label_name() {
        Some(name) => { name },
        None => {
            self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel{instruction: self.current_instruction});
            return;
        }
    };

    // Check if label is already in use (has an entry in the symbol table)
    // TODO: Is there a cleaner way to do this?
    if self.symbols.has_symbol(&name) {
        self.errors.push(AssemblerError::SymbolAlreadyDeclared);
        return;
    }

    // If we make it here, it isn't a symbol we've seen before, so stick it in the table
    let symbol = Symbol::new(name, SymbolType::Label);
    self.symbols.add_symbol(symbol);
}

fn process_second_phase(&mut self, p: &Program) -> Vec<u8>{
    self.current_instruction = 0;
    let mut program = vec![];

    for i in &p.instructions{
        if i.is_opcode() {
            let mut bytes = i.to_bytes(&self.symbols);
            program.append(&mut bytes);
        }
        if i.is_derective (){
            self.process_directive(i);
        }
        self.current_instruction += 1;
    }
    program
}

/// Handles a declaration of a section header, such as:
/// .code
fn process_section_header(&mut self, header_name: &str) {
    let new_section: AssemblerSection = header_name.into();
    // Only specific section names are allowed
    if new_section == AssemblerSection::Unknown {
        println!("Found an section header that is unknown: {:#?}", header_name);
        return;
    }
    // TODO: Check if we really need to keep a list of all sections seen
    self.sections.push(new_section.clone());
    self.current_section = Some(new_section);
}


#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new()
        }
    }
}

/// Handles a declaration of a null-terminated string:
/// hello: .asciiz 'Hello!'
fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
    // Being a constant declaration, this is only meaningful in the first pass
    if self.phase != AssemblerPhase::First { return; }

    // In this case, operand1 will have the entire string we need to read in to RO memory
    match i.get_string_constant() {
        Some(s) => {
            match i.get_label_name() {
                Some(name) => { self.symbols.set_symbol_offset(&name, self.ro_offset); }
                None => {
                    // This would be someone typing:
                    // .asciiz 'Hello'
                    println!("Found a string constant with no associated label!");
                    return;
                }
            };
            // We'll read the string into the read-only section byte-by-byte
            for byte in s.as_bytes() {
                self.ro.push(*byte);
                self.ro_offset += 1;
            }
            // This is the null termination bit we are using to indicate a string has ended
            self.ro.push(0);
            self.ro_offset += 1;
        }
        None => {
            // This just means someone typed `.asciiz` for some reason
            println!("String constant following an .asciiz was empty");
        }
    }
}

Opcode::PRTS => {
    // PRTS takes one operand, either a starting index in the read-only section of the bytecode
    // or a symbol (in the form of @symbol_name), which will look up the offset in the symbol table.
    // This instruction then reads each byte and prints it, until it comes to a 0x00 byte, which indicates
    // termination of the string
    let starting_offset = self.next_16_bits() as usize;
    let mut ending_offset = starting_offset;
    let slice = self.ro_data.as_slice();
    // TODO: Find a better way to do this. Maybe we can store the byte length and not null terminate? Or some form of caching where we
    // go through the entire ro_data on VM startup and find every string and its ending byte location?
    while slice[ending_offset] != 0 {
        ending_offset += 1;
    }
    let result = std::str::from_utf8(&slice[starting_offset..ending_offset]);
    match result {
        Ok(s) => { print!("{}", s); }
        Err(e) => { println!("Error decoding string for prts instruction: {:#?}", e) }
    };
}


#[derive(Debug, PartialEq,Clone)]
pub enum AssemblerPhase {
    First,
    Second,
}

impl Default for AssemblerPhase {
    fn default() -> Self{
        AssemblerPhase::First
    }
}

#[derive(Debug, PartialEq,Clone)]
pub enum AssemblerSection{
    Data { starting_instruction: Option<u32>},
    Code { starting_instruction: Option<u32>},
    Unknown,
}

impl Default for AssemblerSection {
    fn default() -> Self {
        AssemblerSection::Unknown
    }
}

impl<'a> From<&'a str> for AssemblerSection{
    fn from(name: &str) -> AssemblerSection{
        match  name {
            "data" => AssemblerSection::Data { starting_instruction: None },
            "code" => AssemblerSection::Code { starting_instruction: None },
            _ => AssemblerSection::Unknown,
        }
    }
}


fn process_first_phase(&mut self, p: &Program){
    self.extract_labels(p);
    self.phase = AssemblerPhase::Second;
}

fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
    let mut program = vec![];
    for i in &p.instructions {
        let mut bytes = i.to_bytes(&self.symbols);
        program.append(&mut bytes);
    }
    program
}

fn extract_labels(&mut self, p: &Program) {
    let mut c = 0;
    for i in &p.instructions {
        if i.is_label() {
            match i.label_name() {
                Some(name) => {
                    let symbol = Symbol::new(name, SymbolType::Label, c);
                    self.symbols.add_symbol(symbol);
                },
                None => {}
            };
        }
        c += 4;
    }
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name:String,symbol_type,offset:u32)->Symbol{
        Symbol { name, offset , symbol_type }
    }
}


#[derive(Debug)]
pub enum SymbolType{
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>
}

impl SymbolTable {
    pub fn new() -> SymbolTable{
        SymbolTable{
            symbols: vec![]
        }
    }

    pub fn add_symbol(&mut self, s: Symbol){
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str)-> Option<u32>{
        for symbol in &self.symbols{
            if symbol.name == s{
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[test]
fn test_symbol_table() {
    let mut sym = SymbolTable::new();
    let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
    sym.add_symbol(new_symbol);
    assert_eq!(sym.symbols.len(), 1);
    let v = sym.symbol_value("test");
    assert_eq!(true, v.is_some());
    let v = v.unwrap();
    assert_eq!(v, 12);
    let v = sym.symbol_value("does_not_exist");
    assert_eq!(v.is_some(), false);
}

#[test]
fn test_assemble_program() {
    let mut asm = Assembler::new();
    let test_string = "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
    let program = asm.assemble(test_string).unwrap();
    let mut vm = VM::new();
    assert_eq!(program.len(), 21);
    vm.add_bytes(program);
    assert_eq!(vm.program.len(), 21);
}

const PIE_HEADER_PREFIX: [u8;4]=[45,50,49,45];
const PIE_HEADER_LENGTH: usize = 64;

fn write_pie_header(&self) -> Vec<u8>{
    let mut header = vec![];
    for byte in PIE_HEADER_PREFIX.into_iter(){
        header.push(byte.clone());
    }
    while header.len() <= PIE_HEADER_LENGTH{
        header.push(0 as u8);
    }
    header
}

