use crate::lexer::{Lexer, Token};

pub struct Interpreter<Ts>
where
	Ts: Iterator<Item = Token>,
{
	token_stream: Ts,
	stack: Vec<Option<i64>>,
	pointer: Option<usize>,
}

impl<Ts> Interpreter<Ts>
where
	Ts: Iterator<Item = Token>,
{
	pub fn new(token_stream: Ts) -> Self {
		Self {
			token_stream,
			stack: Vec::new(),
			pointer: None,
		}
	}
}
