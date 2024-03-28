use std::{iter::Peekable, str::Chars};

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

	fn consume_integer(&mut self) -> Result<Integer, ()> {
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

		Ok(integer)
	}
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Result<Token, ()>;

	fn next(&mut self) -> Option<Self::Item> {
		let Some(c) = self.source.next() else {
			return None;
		};

		Some(match c {
			'눈' | '누' => Ok(Token::Push),
			'난' | '나' => Ok(Token::Mul),
			'주' => Ok(Token::Sub),
			'거' => Ok(Token::Add),
			'.' => Ok(Token::Dot),
			'헤' => Ok(Token::Pop),
			'으' => Ok(Token::Prev),
			'응' => Ok(Token::PopSub),
			'흐' => unimplemented!(),
			'읏' => unimplemented!(),
			'💕' => Ok(Token::PopAdd),
			'!' => Ok(Token::Print),
			_ => Err(()),
		})
	}
}
