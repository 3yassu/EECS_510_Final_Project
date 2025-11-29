use std::ptr::NonNull;
use core::marker::PhantomData;
use super::terminals::*;
pub struct TuringCell{
    entry: TerminalChar,
    left: CellPtr,
    right: CellPtr,
}
impl TuringCell{
    fn new(entry: TerminalChar) -> Self{
        Self{entry, left: None, right: None}
    }
}
pub type CellPtr = Option<NonNull<TuringCell>>;

pub struct Tape{
    front: CellPtr,
    back: CellPtr,
    len: usize,
    _boo: PhantomData<TerminalChar>
}
impl Tape{
    pub fn new() -> Self{
        Self{front: None, back: None, len: 0, _boo: PhantomData}
    }
    pub fn push_front(&mut self, entry: TerminalChar){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(TuringCell::new(entry))));
            match &mut self.front{
                Some(old) => {
                    (*old.as_ptr()).left = Some(node);
                    (*node.as_ptr()).right = Some(*old);
                },
                None => self.back = Some(node),
            }
            self.front = Some(node);
            self.len += 1;
        }
    }
    pub fn push_back(&mut self, entry: TerminalChar){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(TuringCell::new(entry))));
            match &mut self.back{
                Some(old) => {
                    (*old.as_ptr()).right = Some(node);
                    (*node.as_ptr()).left = Some(*old);
                },
                None => self.front = Some(node),
            }
            self.back = Some(node);
            self.len += 1;
        }
    }
    pub fn pop_front(&mut self) -> Option<TerminalChar>{
        unsafe{
            self.front.map( |node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_entry = boxed.entry;
                self.front = boxed.right;
                match &mut self.front{
                    Some(new) => (*new.as_ptr()).left = None,
                    None => self.back = None,
                }
                self.len -= 1;
                return_entry
            })
        }
    }
    pub fn peek_front_node(&self) -> CellPtr{
		self.front.as_ref().map(|cell| cell.clone())
    }
    pub fn pop_back(&mut self) -> Option<TerminalChar>{
        unsafe{
            self.back.map( |node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_entry = boxed.entry;
                self.back = boxed.left;
                match &mut self.back{
                    Some(new) => (*new.as_ptr()).right = None,
                    None => self.front = None,
                }
                self.len -= 1;
                return_entry
            })
        }
    }
    pub fn clear(&mut self){
        while self.pop_front().is_some(){}
    }
}
impl Drop for Tape {
    fn drop(&mut self){
        while self.pop_front().is_some(){}
    }
}
