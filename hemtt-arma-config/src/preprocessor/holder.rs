use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::tokenizer::tokens::TokenPair;

pub struct Holder<'a> {
    tokens: UnsafeCell<Vec<TokenPair<'a>>>,
    index: AtomicUsize,
}

impl<'a> Holder<'a> {
    pub fn new() -> Self {
        Self {
            tokens: UnsafeCell::new(Vec::with_capacity(10240)),
            index: AtomicUsize::new(0),
        }
    }

    pub fn add(&self, token: TokenPair<'a>) -> Option<usize> {
        let index = self.index.fetch_add(1, Ordering::SeqCst);
        println!("{}", index);
        if index < 10240 {
            // Safety: This is the only time the value can ever be written to
            unsafe {
                let t = &mut *self.tokens.get();
                t.push(token);
            }
            Some(index)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> &'a TokenPair<'a> {
        // Safety: We only ever give out immutable references
        unsafe { &(*self.tokens.get())[index] }
    }
}
