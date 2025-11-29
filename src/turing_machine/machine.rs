//turing_machine/machine.rs
use super::tape::{Tape, CellPtr};
use super::states::State;

struct CTuringMachine{
	tape: Tape,
	cur: CellPtr,
	cur_state: State
}
impl CTuringMachine{
	pub fn new(tape: Tape) -> Self{
		Self{cur: tape.peek_front_node(), tape, cur_state: State::S}
	}
	fn read_char(&mut self){
		self.cur_state.read_char(&mut self.cur)
	}
	pub fn run(&mut self){
		while(self.cur_state != State::H){
			self.read_char();
		}
	}
}
