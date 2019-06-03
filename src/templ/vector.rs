use libc::size_t;

pub trait Vector<'i> {
    type Storage;
    type Arg;

    fn new() -> Self;

    #[inline]
    fn from_iter(s: impl IntoIterator<Item=Self::Arg>) -> Self where Self: Sized {
        let s = s.into_iter();
        let (lo, hi) = s.size_hint();
        let mut out = Self::with_capacity(hi.unwrap_or(lo));
        s.for_each(|x| out.push(x));
        out
    }

    #[inline]
    fn with_capacity(capacity: size_t) -> Self where Self: Sized {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    }

    fn len(&self) -> size_t;

    fn is_empty(&self) -> bool;

    fn capacity(&self) -> size_t;

    fn shrink_to_fit(&mut self);

    fn reserve(&mut self, additional: size_t);

    fn clear(&mut self);

    fn push(&mut self, val: Self::Arg);

    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()>;

    fn remove(&mut self, index: size_t) -> crate::Result<()>;

    fn get(&self, index: size_t) -> crate::Result<Self::Storage>;
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage;

    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()>;
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg);

    #[inline]
    fn to_vec(&self) -> Vec<Self::Storage> {
        (0..self.len()).map(|x| unsafe { self.get_unchecked(x) }).collect()
    }
}

pub struct VectorIterator<T> {
    vec: T,
    i: size_t,
}

impl<T> VectorIterator<T> {
    pub fn new(vec: T) -> Self {
        Self { vec, i: 0 }
    }
}

impl<T, S> Iterator for VectorIterator<T>
    where
        T: for<'i> Vector<'i, Storage=S>
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vec.get(self.i);
        self.i += 1;
        out.ok()
    }
}

pub struct VectorRefIterator<'v, T: 'v> {
    vec: &'v T,
    i: size_t,
}

impl<'v, T> VectorRefIterator<'v, T> {
    pub fn new(vec: &'v T) -> Self {
        Self { vec, i: 0 }
    }
}

impl<T, S> Iterator for VectorRefIterator<'_, T>
    where
        T: for<'i> Vector<'i, Storage=S>,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vec.get(self.i);
        self.i += 1;
        out.ok()
    }
}


