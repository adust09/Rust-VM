pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

#[macro_use]
fn main() {
    println!("Hello, world!");
    let mut repl = repl::REPL::new();
    repl.run();
    extern crate nom;
}
