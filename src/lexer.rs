pub enum Token {
	Push,
	Mul,
	Sub,
	Add,
	Pop,
	Prev,
	PopSub,
	Pow,
	Dots,
}

pub struct Lexer<'s> {
	source: &'s str,
	position: usize,
}

impl<'s> Lexer<'s> {
	pub fn new(source: &'s str) -> Self {
		Self {
			source,
			position: 0,
		}
	}
}
