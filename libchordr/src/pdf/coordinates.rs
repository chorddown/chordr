use printpdf::Mm;

pub struct Coordinates {
    pub(crate) x: Mm,
    pub(crate) y: Mm,
}

impl Coordinates {
    pub fn new(x: Mm, y: Mm) -> Self {
        Self { x, y }
    }
}
