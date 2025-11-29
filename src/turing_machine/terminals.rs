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
	var_type: VariableType,
}

pub enum TerminalChars{
	ParenOpen,
	ParenClose,
	BracketOpen,
	BracketClose,
	Var(Variable)
} 
