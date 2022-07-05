use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use crate::assembler::program_parsers::program;

/// Core structure for the REPL for the Assembler

pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the Assembler REPL!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>>");
            io::stdout().flush().expect("Unable to flush stdout");
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Goodbye!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    // You can assign the result of a match to a variable
                    // Rust can convert types using `Into` and `From`
                    let program = match program(buffer.into()) {
                        // Rusts pattern matching is pretty powerful an can even be nested
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    // The `program` is `pub` anyways so you can just `append` to the `Vec`
                    self.vm.program.append(&mut program.to_bytes());
                }
            }
        }
    }
}
