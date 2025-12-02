//turing_machine/terminals.rs
use std::mem;
#[allow(unused)]
pub enum TerminalChar{
	ParenOpen,
	ParenClose,
	BracketOpen,
	BracketClose,
	SingleQuote,
	DoubleQuote,
	BOF,
	EOF,
	Blank,
	Char(char),
}
impl TerminalChar{
    pub const fn take(&mut self) -> Self {
		//Replace with mem::take with latter is const ready
        mem::replace(self, TerminalChar::Blank)
    }
}
