//turing_machine/machine.rs
//bring the Tape and pointer to Cell in scope as well as States
use super::tape::{Tape, CellPtr};
use super::states::State;
//A turing machine is the Tape, the current position in the tape, and the current State of the machine
#[allow(unused)]
pub struct CTuringMachine{
	tape: Tape,
	cur: CellPtr,
	cur_state: State
}
impl CTuringMachine{
	//creates a new turing machine given a tape (starts at front with start state)
	pub fn new(tape: Tape) -> Self{
		Self{cur: tape.peek_front_node(), tape, cur_state: State::S}
	}
	//Read char, represents a state transition
	fn read_char(&mut self){
		self.cur_state.read_char(&mut self.cur)
	}
	//Reads char until a final state is reaches
	pub fn run(&mut self) -> bool {
		while(self.cur_state != State::A) && (self.cur_state != State::F){
			self.read_char();
		}
		self.cur_state == State::A
	}
}
