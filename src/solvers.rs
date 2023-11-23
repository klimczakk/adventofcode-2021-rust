use std::str::FromStr;

use phf::{Map, phf_map};

#[derive(Debug, Clone)]
pub struct InvalidInputError;

pub type Solver = fn(String) -> Result<i32, InvalidInputError>;

pub const SOLVERS: Map<&'static str, Solver> = phf_map!(
	"1-1" => day1_1 as Solver,
	"1-2" => day1_2,
);

fn day1_1(input: String) -> Result<i32, InvalidInputError> {
	type ParseError = <i32 as FromStr>::Err;

	let parsed_results: Box<[Result<i32, ParseError>]> = input
		.lines()
		.map(str::parse::<i32>)
		.collect();

	if parsed_results.len() == 0 {
		return Err(InvalidInputError);
	}

	if parsed_results.iter().any(Result::is_err) {
		return Err(InvalidInputError);
	}

	let values: Box<[i32]> = parsed_results
		.iter()
		.map(Result::<i32, ParseError>::clone)
		.map(Result::<i32, ParseError>::unwrap)
		.collect();

	Ok(values
		.iter()
		.fold((0, values[0]), |(accumulator, last_value), value| {
			if *value > last_value {
				(accumulator + 1, *value)
			} else {
				(accumulator, *value)
			}
		}).0
	)
}

fn day1_2(input: String) -> Result<i32, InvalidInputError> {
	todo!();
}