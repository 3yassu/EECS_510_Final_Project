//turing_machine/states.rs
//Bring TerminalChar and Tape Location for state transitions 
use super::terminals::TerminalChar;
use super::tape::{CellPtr};
//enum to differentiate where the state is going
pub enum Direction{
	Left,
	Right
}
//State enum
#[derive(PartialEq)]
pub enum State{
	S,
	P,
	B,
	Qd,
	Qs,
	E,
	A,
	F,
}
impl State{
	//State transition, Changes what self is, writes the w_char into CellPtr's location, and moves the required direction
	pub fn read_char(&mut self, cellptr: &mut CellPtr){
		let ptr = cellptr.unwrap();
		let r_char = unsafe{(*ptr.as_ptr()).entry.take()};
		let w_char: TerminalChar;
		let dir: Direction; 
		//Set values and change self
		match self{
			State::S =>
				match &r_char{
					TerminalChar::EOF => (*self, w_char, dir) = (State::E, r_char, Direction::Left),
					TerminalChar::SingleQuote => (*self, w_char, dir) = (State::Qs, TerminalChar::Char(' '), Direction::Right),
					TerminalChar::DoubleQuote => (*self, w_char, dir) = (State::Qd, TerminalChar::Char(' '), Direction::Right),
					TerminalChar::BracketClose => (*self, w_char, dir) = (State::B, TerminalChar::Char(' '), Direction::Left),
					TerminalChar::ParenClose => (*self, w_char, dir) = (State::P, TerminalChar::Char(' '), Direction::Left),
					_ => (w_char, dir) = (r_char, Direction::Right),
				}
			State::P =>
				match &r_char{
					TerminalChar::ParenOpen => (*self, w_char, dir) = (State::S, TerminalChar::Char(' '), Direction::Right),
					TerminalChar::BOF => (*self, w_char, dir) = (State::F, r_char, Direction::Right),
					_ => (w_char, dir) = (r_char, Direction::Left),
				}
			State::B =>
				match &r_char{
					TerminalChar::BracketOpen => (*self, w_char, dir) = (State::S, TerminalChar::Char(' '), Direction::Right),
					TerminalChar::BOF => (*self, w_char, dir) = (State::F, r_char, Direction::Right),
					_ => (w_char, dir) = (r_char, Direction::Left),
				}
			State::Qs =>
				match &r_char{
					TerminalChar::SingleQuote => (*self, w_char, dir) = (State::S, TerminalChar::Char(' '), Direction::Right),
					TerminalChar::EOF => (*self, w_char, dir) = (State::F, r_char, Direction::Left),
					_ => (w_char, dir) = (TerminalChar::Char(' '), Direction::Right),
				}
			State::Qd =>
				match &r_char{
					TerminalChar::DoubleQuote => (*self, w_char, dir) = (State::S, TerminalChar::Char(' '), Direction::Right),
					TerminalChar::EOF => (*self, w_char, dir) = (State::F, r_char, Direction::Left),
					_ => (w_char, dir) = (TerminalChar::Char(' '), Direction::Right),
				}
			State::E =>
				match &r_char{
					TerminalChar::ParenOpen => (*self, w_char, dir) = (State::F, r_char, Direction::Left),
					TerminalChar::BracketOpen => (*self, w_char, dir) = (State::F, r_char, Direction::Left),
					TerminalChar::BOF => (*self, w_char, dir) = (State::A, TerminalChar::BOF, Direction::Right),
					_ => (w_char, dir) = (r_char, Direction::Left),
				}
			//State::A => return true,
			//State::F => return false,
			_ => panic!("This message should not appear, should it? Tell me if you see it."),
		}
		//Write w_char
		unsafe{(*ptr.as_ptr()).entry = w_char};
		//Move direction
		match dir{
			Direction::Left => *cellptr = unsafe{(*ptr.as_ptr()).left},
			Direction::Right => *cellptr = unsafe{(*ptr.as_ptr()).right},
		}
	}
}
