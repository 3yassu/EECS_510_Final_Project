//terminals.rs
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
	Var(Variable)
} 
