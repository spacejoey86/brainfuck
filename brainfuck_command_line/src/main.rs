use brainfuck_lib::{self, program::Program, virtual_machine::VirtualMachine};
use clap::Parser;
use std::{
    fs::{self},
    io::{stdin, stdout},
    path::PathBuf,
};

fn main() {
    let cli = Cli::parse();

    println!("Reading file {:?}", cli.bf_source_file);
    let contents = fs::read_to_string(cli.bf_source_file).expect("Cannot read file");
    let program = Program::from_string(contents);

    let mut input = stdin();
    let mut output = stdout();
    let mut vm = VirtualMachine::new(program, &mut input, &mut output);

    println!("Executing:");
    vm.run().expect("Program execution failed");
}

#[derive(Parser)]
struct Cli {
    bf_source_file: PathBuf,
}
