use std::io::Write;

use crate::lexer::{Lexer, Token};

pub struct Interpreter<Ts, O>
where
	Ts: Iterator<Item = Token>,
	O: Write,
{
	token_stream: Ts,
	stack: Vec<Option<i64>>,
	pointer: Option<usize>,
	output: O,
}

impl<Ts, O> Interpreter<Ts, O>
where
	Ts: Iterator<Item = Token>,
	O: Write,
{
	pub fn new(token_stream: Ts, output: O) -> Self {
		Self {
			token_stream,
			stack: Vec::new(),
			pointer: None,
			output,
		}
	}
}
