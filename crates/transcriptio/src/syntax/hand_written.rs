use std::str::FromStr;

use crate::syntax::ast::Timestamp;

impl Timestamp {
    pub fn as_duration(&self) -> Option<std::time::Duration> {
        todo!();
    }
}

impl FromStr for Timestamp {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    #[ignore = "TODO: implement the parser"]
    fn parse_durations() {
        let inputs = [
            ("1:00:00", Duration::from_secs(60 * 60)),
            ("5:00", Duration::from_secs(5 * 60)),
            (
                "1:00.500",
                Duration::from_secs(60) + Duration::from_millis(500),
            ),
        ];

        for (src, expected) in inputs {
            let timestamp = Timestamp::from_str(src).unwrap();
            assert_eq!(timestamp.as_duration().unwrap(), expected);
        }
    }
}
