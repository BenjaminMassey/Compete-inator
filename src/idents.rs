use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Ident<T> {
    ident: u32,
    phantom: PhantomData<T>,
}

impl<T> Deref for Ident<T> {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.ident
    }
}

pub struct IdentGenerator<T> {
    next_ident: AtomicU32,
    phantom: PhantomData<T>,
}

impl<T> IdentGenerator<T> {
    pub const fn new() -> Self {
        Self {
            next_ident: AtomicU32::new(0),
            phantom: PhantomData,
        }
    }

    pub fn next_ident(&self) -> Ident<T> {
        let ident = self.next_ident.fetch_add(1, SeqCst);
        Ident {
            ident,
            phantom: PhantomData,
        }
    }
}
