use clap::*;

#[derive(Debug)]
pub struct CommandLineInputError;

#[derive(Parser)]
pub struct Args {
	#[arg(help = "[day]-[exercise index] eg: 1-2")]
	pub solver: String,

	#[arg()]
	pub file_path: String,
}