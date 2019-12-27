use crate::parser::SectionType;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Modifier {
    Chorus,
    None,
}

impl Modifier {
    pub fn split(input: &str) -> (Modifier, &str) {
        if input.is_empty() {
            return (Modifier::None, input);
        }

        let (first, rest) = input.split_at(1);

        let modifier = Modifier::from(first);
        if modifier != Modifier::None {
            (modifier, rest)
        } else {
            (modifier, input)
        }
    }
}

impl From<&str> for Modifier {
    fn from(s: &str) -> Self {
        if let Some(first_char) = s.chars().next() {
            Modifier::from(first_char)
        } else {
            Modifier::None
        }
    }
}

impl From<&&str> for Modifier {
    fn from(s: &&str) -> Self {
        if let Some(first_char) = s.chars().next() {
            Modifier::from(first_char)
        } else {
            Modifier::None
        }
    }
}

impl From<char> for Modifier {
    fn from(s: char) -> Self {
        if s == '!' {
            Self::Chorus
        } else {
            Self::None
        }
    }
}

impl From<SectionType> for Modifier {
    fn from(t: SectionType) -> Self {
        match t {
            SectionType::Chorus => Modifier::Chorus,
            SectionType::Verse => Modifier::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(Modifier::split("! Header"), (Modifier::Chorus, " Header"));
        assert_eq!(Modifier::split(" Header"), (Modifier::None, " Header"));
        assert_eq!(Modifier::split("Header"), (Modifier::None, "Header"));
        assert_eq!(Modifier::split(""), (Modifier::None, ""));
        assert_eq!(Modifier::split("!"), (Modifier::Chorus, ""));
        assert_eq!(Modifier::split("! "), (Modifier::Chorus, " "));
    }
}
