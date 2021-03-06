use std::hash::{Hash, Hasher};

use {AlphaEq, Debruijn, FreeName, Pattern, Term};

/// A type annotated with a name for debugging purposes
///
/// The name is ignored for alpha equality comparisons
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Named<N, T> {
    pub name: N,
    pub inner: T,
}

impl<N, T> Named<N, T> {
    pub fn new(name: N, inner: T) -> Named<N, T> {
        Named { name, inner }
    }
}

impl<N, T: AlphaEq> AlphaEq for Named<N, T> {
    fn alpha_eq(&self, other: &Named<N, T>) -> bool {
        T::alpha_eq(&self.inner, &other.inner)
    }
}

impl<T: Term> Term for Named<T::FreeName, T> {
    type FreeName = T::FreeName;
    type BoundName = T::BoundName;

    fn close_at<P>(&mut self, index: Debruijn, pattern: &P)
    where
        P: Pattern<FreeName = Self::FreeName, BoundName = Self::BoundName>,
    {
        self.inner.close_at(index, pattern);
    }

    fn open_at<P>(&mut self, index: Debruijn, pattern: &P)
    where
        P: Pattern<FreeName = Self::FreeName, BoundName = Self::BoundName>,
    {
        self.inner.open_at(index, pattern);
    }
}

impl<N: FreeName, T> Pattern for Named<N, T>
where
    T: Term<FreeName = N, BoundName = Debruijn>,
{
    type NamePerm = N;

    fn freshen(&mut self) -> N {
        self.name.freshen();
        self.name.clone()
    }

    fn rename(&mut self, perm: &N) {
        self.name = perm.clone(); // FIXME: double clone
    }

    fn on_free(&self, index: Debruijn, name: &Self::FreeName) -> Option<Debruijn> {
        match *name == self.name {
            true => Some(index),
            false => None,
        }
    }

    fn on_bound(&self, index: Debruijn, name: &Debruijn) -> Option<Self::FreeName> {
        match *name == index {
            true => Some(self.name.clone()),
            false => None,
        }
    }
}

impl<N, T: Hash> Hash for Named<N, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}
