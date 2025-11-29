//turing_machine/states.rs
use super::terminals::TerminalChar;
use super::tape::{TuringCell, CellPtr};
pub enum Direction{
	Left,
	Right
}
#[derive(PartialEq)]
pub enum State{
	S,
	H,
}
impl State{
	pub fn read_char(&mut self, cellptr: &mut CellPtr){
		let ptr = cellptr.unwrap();
		let r_char = unsafe{(*ptr.as_ptr()).entry.take()};
		let w_char: TerminalChar;
		let dir: Direction; 
		match self{
			State::S =>
				match &r_char{
					TerminalChar::Blank | TerminalChar::EOF => (*self, w_char, dir) = (State::H, r_char, Direction::Left),
					_ => todo!(),
				}
			_ => todo!(),
		}
		unsafe{(*ptr.as_ptr()).entry = w_char};
		match dir{
			Direction::Left => *cellptr = unsafe{(*ptr.as_ptr()).left},
			Direction::Right => *cellptr = unsafe{(*ptr.as_ptr()).right},
		}
	}
}
