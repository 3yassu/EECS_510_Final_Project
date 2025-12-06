//turing_machine/terminals.rs
//needed for function
use std::mem;
//TerminalChar is an enum of all possible TerminalCharacters
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
	//Makes it so clone is never requires, sneaky little trick
    pub const fn take(&mut self) -> Self {
		//Replace with mem::take with latter is const ready
        mem::replace(self, TerminalChar::Blank)
    }
}
