use log::trace;
use RepeatState::{Digits, MatchEnd, Multiplier, NoMatch, Start, Whitespace};

#[derive(Debug, PartialEq, Copy, Clone)]
pub(super) enum RepeatState {
    Start,
    Digits,
    Multiplier,
    Whitespace,
    MatchEnd,
    NoMatch,
}

#[derive(Debug, PartialEq)]
pub(super) struct RepeatMatch {
    /// Start of the match
    pub capture_start: usize,

    /// String buffer for the matching group (e.g. prefixes like `4x `, `13* `, or a suffix ` 5*`)
    pub capture: String,

    /// String buffer for the matching digits only
    pub digit_capture: String,
}

pub(super) struct StateMachine {}

impl StateMachine {
    pub(super) fn new() -> Self {
        Self {}
    }

    pub(super) fn matches(&self, value: &str) -> Result<RepeatMatch, String> {
        if value.is_empty() {
            return Err("Value is empty".to_string());
        }

        let mut state = Start;
        let mut last_state = Start;

        // String buffer for the matching group (e.g. prefixes like `4x `, `13* `, or a suffix ` 5*`)
        let mut capture = String::new();

        // String buffer for the matching digits only
        let mut digit_capture = String::new();

        // Start of the match
        let mut capture_start: Option<usize> = None;

        let count = value.chars().count();
        for (index, character) in value.chars().enumerate() {
            capture.push(character);
            state = self.visit(character, state, &mut digit_capture, index == count - 1);
            trace!(
                "New state for character: {}: {:?}  / Old: {:?}",
                character,
                state,
                last_state
            );

            if state == MatchEnd {
                // The pattern was found -> break out of the loop and stop processing the rest
                break;
            }
            if state == NoMatch {
                // Clear the captures
                digit_capture.clear();
                capture.clear();
            }
            if (state == Digits || state == Multiplier) && capture_start.is_none() {
                capture_start = Some(index);
            }
            last_state = state;
        }

        if state != MatchEnd {
            return Err("No match".to_string());
        }

        assert!(
            capture_start.is_some(),
            "State is {:?}, but capture_start is None",
            state
        );
        let capture_start = capture_start.expect("Capture start not set");

        trace!(
            "Capture: '{}' | Digit capture: '{}' | Capture start: {:?}",
            capture,
            digit_capture,
            capture_start
        );

        Ok(RepeatMatch {
            capture,
            digit_capture,
            capture_start,
        })
    }

    fn visit(
        &self,
        character: char,
        state: RepeatState,
        capture: &mut String,
        is_last_character: bool,
    ) -> RepeatState {
        match state {
            Start => {
                match character {
                    '0'..='9' => {
                        capture.push(character);

                        Digits
                    }
                    _ if character.is_whitespace() => Whitespace, // Ignore the whitespace
                    _ => NoMatch,
                }
            }
            Digits => match character {
                '0'..='9' => {
                    capture.push(character);

                    Digits
                }
                '*' | 'x' => {
                    if is_last_character {
                        MatchEnd
                    } else {
                        Multiplier
                    }
                }
                _ => NoMatch,
            },
            Multiplier => {
                if character.is_whitespace() {
                    MatchEnd
                } else {
                    Whitespace
                }
            }
            Whitespace => match character {
                '0'..='9' => {
                    capture.push(character);

                    Digits
                }
                _ if character.is_whitespace() => Whitespace,
                _ => NoMatch,
            },
            NoMatch => {
                if character.is_whitespace() {
                    Whitespace
                } else {
                    NoMatch
                }
            }
            MatchEnd => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    impl RepeatMatch {
        fn new<S1: Into<String>, S2: Into<String>>(
            capture_start: usize,
            capture: S1,
            digit_capture: S2,
        ) -> Self {
            Self {
                capture: capture.into(),
                digit_capture: digit_capture.into(),
                capture_start,
            }
        }
    }

    #[test]
    fn match_prefix_test() {
        let state_machine = StateMachine::new();
        assert_eq!(
            RepeatMatch::new(0, "2x ", "2"),
            state_machine.matches("2x Chorus").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(0, "5* ", "5"),
            state_machine.matches("5* Chorus").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(0, "5x ", "5"),
            state_machine.matches("5x My Chorus").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(0, "5* ", "5"),
            state_machine.matches("5* My Chorus").unwrap()
        );
        // not implemented yet
        // assert_eq!(
        //     RepeatMatch::new(5, "My Chorus".try_into().unwrap()),
        //     detector.matches("x5 My Chorus").unwrap()
        // );
        // assert_eq!(
        //     RepeatMatch::new(5, "My Chorus".try_into().unwrap()),
        //     detector.matches("*5 My Chorus").unwrap()
        // );
        assert_eq!(
            RepeatMatch::new(0, "172* ", "172"),
            state_machine.matches("172* My Chorus").unwrap()
        );
    }

    #[test]
    fn match_suffix_test() {
        let detector = StateMachine::new();
        assert_eq!(
            RepeatMatch::new(7, " 2x", "2"),
            detector.matches("Chorus 2x").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(7, " 5*", "5"),
            detector.matches("Chorus 5*").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(10, " 5x", "5"),
            detector.matches("My Chorus 5x").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(10, " 5*", "5"),
            detector.matches("My Chorus 5*").unwrap()
        );
        assert_eq!(
            RepeatMatch::new(10, " 156*", "156"),
            detector.matches("My Chorus 156*").unwrap()
        );
    }

    #[test]
    fn match_should_fail_test() {
        let state_machine = StateMachine::new();
        // assert!(state_machine.matches("2x").is_err());
        assert!(state_machine.matches("2").is_err());
        assert!(state_machine.matches("x").is_err());
        // assert!(state_machine.matches("2*").is_err());
        assert!(state_machine.matches("*").is_err());
        assert!(state_machine.matches("Hallo").is_err());
        assert!(state_machine.matches(" ").is_err());
        assert!(state_machine.matches("").is_err());
        assert!(state_machine.matches("3xtra cool").is_err());
        assert!(state_machine.matches("Nice tr4x").is_err());
    }
}
