use std::error::Error;

pub enum Tri<T, E: Error> {
    Some(T),
    None,
    Err(E),
}

#[allow(unused)]
impl<T, E: Error> Tri<T, E> {
    pub fn is_some(&self) -> bool {
        match self {
            Tri::Some(_) => true,
            Tri::None => false,
            Tri::Err(_) => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Tri::Some(_) => false,
            Tri::None => true,
            Tri::Err(_) => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            Tri::Some(_) => false,
            Tri::None => false,
            Tri::Err(_) => true,
        }
    }

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`None`] or [`Err`].
    pub fn unwrap(self) -> T {
        match self {
            Tri::Some(v) => v,
            Tri::None => panic!("called `Tri::unwrap()` on a `None` value"),
            Tri::Err(_) => panic!("called `Tri::unwrap()` on a `Err` value"),
        }
    }
}

impl<T: Clone, E: Error + Clone> Clone for Tri<T, E> {
    fn clone(&self) -> Self {
        match self {
            Tri::Some(v) => Self::Some(v.clone()),
            Tri::None => Self::None,
            Tri::Err(e) => Self::Err(e.clone()),
        }
    }
}

impl<T, E: Error> From<Result<T, E>> for Tri<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(inner) => Self::Some(inner),
            Err(e) => Self::Err(e),
        }
    }
}
