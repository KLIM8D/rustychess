use std::borrow::Borrow;
use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::pieces::Piece;

pub trait KeyPair<A, B> {
    /// Obtains the first element of the pair.
    fn a(&self) -> &A;
    /// Obtains the second element of the pair.
    fn b(&self) -> &B;
}

impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

impl<A: Hash, B: Hash> Hash for (dyn KeyPair<A, B> + '_) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.b().hash(state);
    }
}

impl<A: Eq, B: Eq> PartialEq for (dyn KeyPair<A, B> + '_) {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl<A: Eq, B: Eq> Eq for (dyn KeyPair<A, B> + '_) {}

pub struct Chessboard<A: Eq + Hash, B: Eq + Hash> {
    map: HashMap<(A, B), Box<dyn Any>>,
}

impl<A: Eq + Hash, B: Eq + Hash> Chessboard<A, B> {
    pub fn new() -> Self {
        Chessboard {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, a: &A, b: &B) -> Option<&Piece> {
        self.map.get(&(a, b) as &dyn KeyPair<A, B>)
            .map(|v| v.downcast_ref().unwrap())
    }

    pub fn set(&mut self, a: A, b: B, v: Box<Piece>) {
        self.map.insert((a, b), v);
    }
}

impl<A, B> KeyPair<A, B> for (A, B) {
    fn a(&self) -> &A {
        &self.0
    }
    fn b(&self) -> &B {
        &self.1
    }
}

impl<A, B> KeyPair<A, B> for (&A, &B) {
    fn a(&self) -> &A {
        self.0
    }
    fn b(&self) -> &B {
        self.1
    }
}

//----------------------------------------------------------------

#[derive(Eq, PartialEq, Hash)]
pub struct A(&'static str);

impl A {
    pub const fn new(p: &'static str) -> A {
        A(p)
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct B(&'static i8);

impl B {
    pub const fn new(p: &'static i8) -> B {
        B(p)
    }
}
