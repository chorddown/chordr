#[deprecated(note = "Use modification::transposition::TransposableTrait")]
pub trait TransposableTrait {
    fn transpose(self, semitones: isize) -> Self;
}
