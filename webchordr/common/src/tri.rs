use std::fmt::{Debug, Display, Formatter};

pub enum Tri<T, E> {
    Some(T),
    None,
    Err(E),
}

#[allow(unused)]
impl<T, E> Tri<T, E> {
    pub fn from_option(input: Option<T>) -> Self {
        match input {
            None => Self::None,
            Some(v) => Self::Some(v),
        }
    }

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

impl<T: Clone, E: Clone> Clone for Tri<T, E> {
    fn clone(&self) -> Self {
        match self {
            Tri::Some(v) => Self::Some(v.clone()),
            Tri::None => Self::None,
            Tri::Err(e) => Self::Err(e.clone()),
        }
    }
}

fn matches_some<T: PartialEq, E>(this: &Tri<T, E>, other: &T) -> bool {
    if let Tri::Some(s) = this {
        other == s
    } else {
        false
    }
}

fn matches_error<T, E: PartialEq>(this: &Tri<T, E>, other: &E) -> bool {
    if let Tri::Err(s) = this {
        other == s
    } else {
        false
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for Tri<T, E> {
    fn eq(&self, other: &Self) -> bool {
        match other {
            Tri::Some(i) => matches_some(self, i),
            Tri::None => matches!(self, Tri::None),
            Tri::Err(e) => matches_error(self, e),
        }
    }
}

impl<T: Debug, E: Debug> Debug for Tri<T, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tri::Some(i) => write!(f, "{:?}", i),
            Tri::None => write!(f, "None"),
            Tri::Err(i) => write!(f, "{:?}", i),
        }
    }
}

impl<T: Display, E: Display> Display for Tri<T, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tri::Some(i) => write!(f, "{}", i),
            Tri::None => write!(f, "None"),
            Tri::Err(i) => write!(f, "{}", i),
        }
    }
}

impl<T, E> From<Option<T>> for Tri<T, E> {
    fn from(value: Option<T>) -> Self {
        Self::from_option(value)
    }
}

// `From<webchordr_common::errors::PersistenceError>
impl<T> From<crate::errors::PersistenceError> for Tri<T, crate::errors::PersistenceError> {
    fn from(value: crate::errors::PersistenceError) -> Self {
        Self::Err(value)
    }
}

impl<T, E> From<Result<Option<T>, E>> for Tri<T, E> {
    fn from(result: Result<Option<T>, E>) -> Self {
        match result {
            Ok(inner) => match inner {
                Some(inner) => Self::Some(inner),
                None => Self::None,
            },
            Err(e) => Self::Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for Tri<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(inner) => Self::Some(inner),
            Err(e) => Self::Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tri::Tri;

    #[test]
    fn from_option_test() {
        let option: Option<u8> = Some(8);
        let expected: Tri<u8, u8> = Tri::Some(8);
        assert_eq!(expected, Tri::from(option));

        let option: Option<u8> = None;
        let expected: Tri<u8, u8> = Tri::None;
        assert_eq!(expected, Tri::from(option));
    }

    #[test]
    fn from_result_test() {
        let option: Result<u8, ()> = Ok(8);
        let expected: Tri<u8, _> = Tri::Some(8);
        assert_eq!(expected, Tri::from(option));

        let option: Result<u8, u8> = Err(1);
        let expected: Tri<_, _> = Tri::Err(1u8);
        assert_eq!(expected, Tri::from(option));
    }

    #[test]
    fn from_option_result_test() {
        let option: Result<Option<u8>, u8> = Ok(Some(8));
        let expected: Tri<u8, _> = Tri::Some(8);
        assert_eq!(expected, Tri::from(option));

        let option: Result<Option<u8>, u8> = Ok(None);
        let expected: Tri<u8, _> = Tri::None;
        assert_eq!(expected, Tri::from(option));

        let option: Result<Option<u8>, u8> = Err(1);
        let expected: Tri<u8, _> = Tri::Err(1);
        assert_eq!(expected, Tri::from(option));
    }
}
