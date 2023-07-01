use std::cmp::Ordering;
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

impl<T: std::cmp::Eq> Ord for Ident<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ident.cmp(&other.ident)
    }
}

impl<T: std::cmp::PartialEq> PartialOrd for Ident<T> {
    // Required method
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ident.cmp(&other.ident))
    }

    // Provided methods
    fn lt(&self, other: &Self) -> bool {
        self.ident < other.ident
    }
    fn le(&self, other: &Self) -> bool {
        self.ident <= other.ident
    }
    fn gt(&self, other: &Self) -> bool {
        self.ident > other.ident
    }
    fn ge(&self, other: &Self) -> bool {
        self.ident >= other.ident
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
