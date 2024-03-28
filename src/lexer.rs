use std::{iter::Peekable, str::Chars};

use crate::error::NunaError;

pub enum Token {
	Push(Integer),
	Pop,
	Add(Integer),
	Sub(Integer),
	Mul(Integer),
	Pow(Integer),
	PopAdd,
	PopSub,
	Print,
}

pub struct Integer {
	pub literal: usize,
	pub prevs: usize,
}

pub struct Lexer<'s> {
	source: Peekable<Chars<'s>>,
}

impl<'s> Lexer<'s> {
	pub fn new(source: &'s str) -> Self {
		Self {
			source: source.chars().peekable(),
		}
	}

	fn consume_integer(&mut self) -> Integer {
		let mut integer = Integer {
			literal: 0,
			prevs: 0,
		};

		while let Some(c) = self.source.peek() {
			match c {
				'.' => {
					integer.literal += 1;
					self.source.next().unwrap();
				},
				'으' => {
					integer.prevs += 1;
					self.source.next().unwrap();
				},
				_ => break,
			}
		}

		if matches!(
			integer,
			Integer {
				literal: 0,
				prevs: 0
			}
		) {
			integer.literal = 1;
		}

		integer
	}
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Result<Token, NunaError>;

	fn next(&mut self) -> Option<Self::Item> {
		let Some(c) = self.source.next() else {
			return None;
		};

		Some(match c {
			'눈' | '누' => Ok(Token::Push(self.consume_integer())),
			'난' | '나' => Ok(Token::Mul(self.consume_integer())),
			'주' => Ok(Token::Sub(self.consume_integer())),
			'거' => Ok(Token::Add(self.consume_integer())),
			'헤' => Ok(Token::Pop),
			'응' => Ok(Token::PopSub),
			'흐' => {
				let c = self.consume_integer();

				match self.source.next() {
					Some('읏') => Ok(Token::Pow(c)),
					_ => Err(NunaError::SyntaxError),
				}
			},
			'💕' => Ok(Token::PopAdd),
			'!' => Ok(Token::Print),
			_ => Err(NunaError::SyntaxError),
		})
	}
}
