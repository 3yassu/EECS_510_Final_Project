use super::tape::{Tape, CellPtr};

struct CTuringMachine{
	tape: Tape,
	cur: CellPtr,	
}
impl CTuringMachine{
	pub fn new(tape: Tape) -> Self{
		Self{cur: tape.peek_front_node(), tape}
	} 
}
