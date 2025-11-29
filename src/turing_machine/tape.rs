use std::cmp::Ordering;
use std::NonNull;
use core::marker::PhantomData;
struct TuringCell<T>{
    entry: T,
    left: CellPtr<T>
    right: CellPtr<T>
}
impl<T> Node<T>{
    fn new(entry: T) => Self{
        Self{entry, forw: None, back: None}
    }
}
type CellPtr<T> = Option<NonNull<TuringCell<T>>>;

pub struct Tape<T>{
    front: CellPtr<T>,
    back: CellPtr<T>,
    len: usize,
    _boo: PhantomData<T>
}
impl<T> Tape<T>{
    pub fn new() -> Self{
        Self{front: None, back: None, len: 0, _boo: PhantomData}
    }
    pub fn push_front(&mut self, entry: T){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(entry))));
            match &mut self.front{
                Some(old) => {
                    (*old.as_ptr()).back = Some(new);
                    (*new.as_ptr()).forw = Some(old);
                },
                None => self.back = Some(node),
            }
            self.front = Some(node);
            self.len += 1;
        }
    }
    pub fn push_back(&mut self, entry: T){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(entry))));
            match &mut self.back{
                Some(old) => {
                    (*old.as_ptr()).forw = Some(new);
                    (*new.as_ptr()).back = Some(old);
                },
                None => self.front = Some(node),
            }
            self.back = Some(node);
            self.len += 1;
        }
    }
    pub fn pop_front(&mut self) -> Option<T>{
        unsafe{
            self.front.map( |node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_entry = boxed.entry;
                self.front = boxed.forw;
                match &mut self.front{
                    Some(new) => (*new.as_ptr()).back = None,
                    None => self.back = None,
                }
                self.len -= 1;
                return_entry
            })
        }
    }
    pub fn pop_back(&mut self) -> Option<T>{
        unsafe{
            self.back.map( |node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_entry = boxed.entry;
                self.front = boxed.forw;
                match &mut self.front{
                    Some(new) => (*new.as_ptr()).forw = None,
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
impl<T> Drop for Tape<T> {
    fn drop(&mut self){
        while self.pop_front().is_some(){}
    }
}
