use std::{str::FromStr, iter::repeat};

use phf::{Map, phf_map};

#[derive(Debug, Clone)]
pub struct InvalidInputError;

pub type Solver = fn(String) -> Result<i32, InvalidInputError>;

pub const SOLVERS: Map<&'static str, Solver> = phf_map!(
	"1-1" => day1_1 as Solver,
	"1-2" => day1_2,
	"2-1" => day2_1,
	"2-2" => day2_2,
	"3-1" => day3_1,
);

fn day1_1(input: String) -> Result<i32, InvalidInputError> {
	let values = string_to_i32s(input)?;

	Ok(num_of_increments(&values))
}

fn day1_2(input: String) -> Result<i32, InvalidInputError> {
	let values = string_to_i32s(input)?;

	let sums: Box<[i32]> = values
		.windows(3)
		.map(|cells| cells.iter().sum())
		.collect();

	Ok(num_of_increments(&sums))
}

fn day2_1(input: String) -> Result<i32, InvalidInputError> {
	let [mut depth, mut distance] = [0, 0];

	for line in input.lines() {
		let	split: Box<[&str]> = line.split(' ').collect();

		let command: &str = match split.first() {
			Some(value) => value,
			None => return Err(InvalidInputError),
		};

		let stringified_value: &str = match split.last() {
			Some(value) => value,
			None => return Err(InvalidInputError),
		};

		let value = match stringified_value.parse::<i32>() {
			Ok(value) => value,
			Err(_) => return Err(InvalidInputError),
		};

		match command {
			"forward" => distance += value,
			"down" => depth += value,
			"up" => depth -= value,
			_ => return Err(InvalidInputError),
		}
	}

	Ok(depth * distance)
}

fn day2_2(input: String) -> Result<i32, InvalidInputError> {
	let [mut aim, mut depth, mut distance] = [0, 0, 0];

	for line in input.lines() {
		let split: Box<[&str]> = line.split(' ').collect();

		let command: &str = match split.first() {
			Some(value) => value,
			None => return Err(InvalidInputError),
		};

		let stringified_value: &str = match split.last() {
			Some(value) => value,
			None => return Err(InvalidInputError),
		};

		let value = match stringified_value.parse::<i32>() {
			Ok(value) => value,
			Err(_) => return Err(InvalidInputError),
		};

		match command {
			"forward" => {
				distance += value;
				depth += aim * value;
			},
			"down" => aim += value,
			"up" => aim -= value,
			_ => return Err(InvalidInputError),
		}
	}
	Ok(distance * depth)
}

fn day3_1(input: String) -> Result<i32, InvalidInputError> {
	let mut ones_counter: Box<[u64]> = repeat(0).take(12).collect();

	for line in input.lines() {
		for position in (0..line.len()).rev() {
			let chars = line.chars().collect::<Box<[char]>>();
			if chars[position] == '1' {
				ones_counter[line.len() - 1 - position] += 1;
			}
		}
	}

	let mut gamma: i32 = 0;

	for index in 0..ones_counter.len() {
		if (input.lines().count() as u64 - ones_counter[index]) < ones_counter[index] {
			gamma |= 1 << index;
		}
	}

	let epsilon = 0b111111111111 ^ gamma;

	Ok(gamma * epsilon)
}

fn num_of_increments(values: &[i32]) -> i32 {
	values
		.iter()
		.fold((0, values[0]), |(accumulator, last_value), value| {
			if *value > last_value {
				(accumulator + 1, *value)
			} else {
				(accumulator, *value)
			}
		})
		.0
}

fn string_to_i32s(input: String) -> Result<Box<[i32]>, InvalidInputError> {
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

	Ok(values)
}