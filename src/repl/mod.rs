pub mod command_parser;

use nom::types::CompleteStr;
use repl::command_parser::CommandParser;
use std;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;

use assembler::program_parsers::program;
use assembler::Assembler;
use nom::types::Input;
use scheduler::Scheduler;
use vm::VM;
/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
    scheduler: Scheduler,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
            asm: Assembler::new(),
            scheduler: Scheduler::new(),
        }
    }

    /// Run loop similar to the VM execution loop, but the instructions are taken from the user directly
    /// at the terminal and not from pre-compiled bytecode
    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how allocate this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let historical_copy = buffer.clone();
            self.command_buffer.push(historical_copy);

            if buffer.starts_with("!") {
                self.execute_command(&buffer);
            } else {
                let program = match program(CompleteStr(&buffer)) {
                    Ok((_remainder, program)) => program,
                    Err(e) => {
                        println!("Unable to parse input: {:?}", e);
                        continue;
                    }
                };
                self.vm
                    .program
                    .append(&mut program.to_bytes(&self.asm.symbols));
                self.vm.run_once();
            }
        }
    }

    fn get_data_from_load(&mut self) -> Option<String> {
        let stdin = io::stdin();
        print!("Please enter the path to the file you wish to load: ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut tmp = String::new();

        stdin
            .read_line(&mut tmp)
            .expect("Unable to read line from user");
        println!("Attempting to load program from file...");

        let tmp = tmp.trim();
        let filename = Path::new(&tmp);
        let mut f = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => {
                println!("There was an error opening that file: {:?}", e);
                return None;
            }
        };
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Ok(_bytes_read) => Some(contents),
            Err(e) => {
                println!("there was an error reading that file: {:?}", e);
                None
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}

fn execute_command(&mut self, input: &str) {
    let args = CommandParser::tokenize(input);
    match args[0] {
        "!quit" => self.quit(&args[1..]),
        "!history" => self.history(&args[1..]),
        "!program" => self.program(&args[1..]),
        "!clear_program" => self.clear_program(&args[1..]),
        "!clear_registers" => self.clear_registers(&args[1..]),
        "!registers" => self.registers(&args[1..]),
        "!symbols" => self.symbols(&args[1..]),
        "!load_file" => self.load_file(&args[1..]),
        "!spawn" => self.spawn(&args[1..]),
        _ => {
            println!("Invalid command")
        }
    };
}
