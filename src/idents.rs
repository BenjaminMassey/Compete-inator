use std::marker::PhantomData;
use std::ops::Deref;

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

#[derive(Default)]
pub struct IdentGenerator<T> {
    next_ident: u32,
    phantom: PhantomData<T>,
}

impl<T> Iterator for IdentGenerator<T> {
    type Item = Ident<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(Ident {
            ident: self.next_ident,
            phantom: PhantomData,
        });
        self.next_ident += 1;
        result
    }
}
