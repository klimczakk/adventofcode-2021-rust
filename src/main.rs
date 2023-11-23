use std::{fs::read_to_string, process::ExitCode};

use args::Args;
use clap::Parser;
use solvers::*;


mod args;
mod solvers;
mod error {
	use std::io;
    use crate::solvers::InvalidInputError;

	#[derive(Debug)]
	pub struct CommandLineInputError;

	#[derive(Debug)]
	pub enum MainError {
		InvalidInput(InvalidInputError),
		IO(io::Error),
	}

	impl From<io::Error> for MainError {
		fn from(value: io::Error) -> Self {
			MainError::IO(value)
		}
	}

	impl From<InvalidInputError> for MainError {
		fn from(value: InvalidInputError) -> Self {
			MainError::InvalidInput(value)
		}
	}
}

fn main() -> ExitCode {
    let args = Args::parse();

	let input: String = match read_to_string(&args.file_path) {
		Ok(value) => value,
		Err(_) => {
			eprintln!("Could not access input data from file {}!", &args.file_path);
			return ExitCode::FAILURE;
		}
	};

	if let Some(solver) = SOLVERS.get(args.solver.as_str()) {
		match solver(input) {
			Ok(result) => println!("{}", result),
			Err(_) => {
				eprintln!("Data inside input file is corrupted!");
				return ExitCode::FAILURE;
			}
		}
	} else {
		eprintln!("Invalid solver signature {} specified!", &args.solver);
		return ExitCode::FAILURE;
	}

	ExitCode::SUCCESS
}
