pub enum FromHeaderResult<T, E> {
    None,
    Ok(T),
    Err(E),
}

pub trait FromHeader: Sized {
    type Err;

    /// Try to create an instance of `Self` from the given headers
    fn from_headers(headers: Vec<&str>) -> FromHeaderResult<Self, Self::Err> {
        for header in headers {
            match Self::from_header(header) {
                FromHeaderResult::None => { /* continue */ }
                FromHeaderResult::Ok(c) => return FromHeaderResult::Ok(c),
                FromHeaderResult::Err(e) => return FromHeaderResult::Err(e),
            }
        }

        FromHeaderResult::None
    }

    /// Try to create an instance of `Self` from the given header
    fn from_header(header: &str) -> FromHeaderResult<Self, Self::Err>;
}
