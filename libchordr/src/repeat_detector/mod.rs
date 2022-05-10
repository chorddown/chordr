//! Repeat Detector scans texts for patterns like
//!  - 2x Chorus 1
//!  - 2* Bridge
//!  - Interlude 6x
//! resembling regular expressions like `^\d+(x|\*)\s` and `\s\d+(x|\*)$`
use crate::models::structure::SectionIdentifier;
use crate::repeat_detector::state_machine::{RepeatMatch, StateMachine};
mod state_machine;

#[derive(Debug, PartialEq)]
pub(crate) struct RepeatInfo {
    pub count: usize,
    pub identifier: SectionIdentifier,
}

impl RepeatInfo {
    fn new(count: usize, identifier: SectionIdentifier) -> Self {
        Self { count, identifier }
    }
}

#[derive(Default)]
pub(crate) struct RepeatDetector {}

impl RepeatDetector {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn detect(&self, value: &str) -> Result<RepeatInfo, String> {
        let repeat_match = StateMachine::new().matches(value)?;

        let RepeatMatch {
            capture_start,
            capture,
            digit_capture,
        } = repeat_match;

        let repeat_count = match digit_capture.parse::<usize>() {
            Ok(c) => c,
            Err(e) => return Err(format!("Parsing the count failed: {}", e)),
        };

        let identifier = self.get_identifier_for_capture(value, &capture, capture_start)?;

        Ok(RepeatInfo::new(repeat_count, identifier))
    }

    fn get_identifier_for_capture(
        &self,
        value: &str,
        capture: &str,
        capture_start: usize,
    ) -> Result<SectionIdentifier, String> {
        match SectionIdentifier::try_from(self.get_substring_for_identifier(
            value,
            capture_start,
            &capture,
        )) {
            Ok(i) => Ok(i),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_substring_for_identifier(
        &self,
        value: &str,
        capture_start: usize,
        capture: &str,
    ) -> String {
        if capture_start == 0 {
            // `value` starts with `capture` -> Remove the prefix
            substring_from(value, capture.chars().count())
        } else {
            substring_to(value, capture_start)
        }
    }
}

fn substring_to(value: &str, end: usize) -> String {
    value.chars().take(end).collect()
}

fn substring_from(value: &str, start: usize) -> String {
    value
        .chars()
        .skip(
            start, // last_index +
                  //     1 + // Multiplier
                  //     1, // Whitespace
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_prefix_test() {
        let detector = RepeatDetector::new();
        assert_eq!(
            RepeatInfo::new(2, "Chorus".try_into().unwrap()),
            detector.detect("2x Chorus").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(5, "Chorus".try_into().unwrap()),
            detector.detect("5* Chorus").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(5, "My Chorus".try_into().unwrap()),
            detector.detect("5x My Chorus").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(5, "My Chorus".try_into().unwrap()),
            detector.detect("5* My Chorus").unwrap()
        );
        // not implemented yet
        // assert_eq!(
        //     RepeatInfo::new(5, "My Chorus".try_into().unwrap()),
        //     detector.detect("x5 My Chorus").unwrap()
        // );
        // assert_eq!(
        //     RepeatInfo::new(5, "My Chorus".try_into().unwrap()),
        //     detector.detect("*5 My Chorus").unwrap()
        // );
        assert_eq!(
            RepeatInfo::new(172, "My Chorus".try_into().unwrap()),
            detector.detect("172* My Chorus").unwrap()
        );
    }

    #[test]
    fn detect_suffix_test() {
        let detector = RepeatDetector::new();
        assert_eq!(
            RepeatInfo::new(2, "Chorus".try_into().unwrap()),
            detector.detect("Chorus 2x").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(5, "Chorus".try_into().unwrap()),
            detector.detect("Chorus 5*").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(5, "My Chorus".try_into().unwrap()),
            detector.detect("My Chorus 5x").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(5, "My Chorus".try_into().unwrap()),
            detector.detect("My Chorus 5*").unwrap()
        );
        assert_eq!(
            RepeatInfo::new(156, "My Chorus".try_into().unwrap()),
            detector.detect("My Chorus 156*").unwrap()
        );
    }

    #[test]
    fn detect_should_fail_test() {
        let detector = RepeatDetector::new();
        assert!(detector.detect("2x").is_err());
        assert!(detector.detect("2").is_err());
        assert!(detector.detect("x").is_err());
        assert!(detector.detect("2*").is_err());
        assert!(detector.detect("*").is_err());
        assert!(detector.detect("Hallo").is_err());
        assert!(detector.detect(" ").is_err());
        assert!(detector.detect("").is_err());
        assert!(detector.detect("3xtra cool").is_err());
        assert!(detector.detect("Nice tr4x").is_err());
    }
}
