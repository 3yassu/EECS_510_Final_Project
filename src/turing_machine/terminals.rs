//turing_machine/terminals.rs
use std::mem;
pub enum VariableType{
	Char,
	Int,
	Short,
	Long,
	Longx2,
	Float,
	Double,
	LongDouble
}
pub struct Variable{
	is_pointer: bool,
	is_global: bool,
	is_static: bool,
	init: bool,
	var_type: VariableType,
	name: String
}

pub enum TerminalChar{
	ParenOpen,
	ParenClose,
	BracketOpen,
	BracketClose,
	Var(Variable),
	EOF,
	Blank,
}
impl TerminalChar{
    pub const fn take(&mut self) -> Self {
		//Replace with mem::take with latter is const ready
        mem::replace(self, TerminalChar::Blank)
    }
}
