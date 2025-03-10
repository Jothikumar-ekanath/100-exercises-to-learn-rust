// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.
#![allow(unused)]
#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}
#[derive(thiserror::Error, Debug,PartialEq)]
#[error("{invalid_string} not a valid Status string")] // This provides a to_string() implementation
struct ParseError{
    invalid_string: String,
}
impl ParseError {
    fn new(invalid_string: String) -> Self {
        Self { invalid_string }
    }
}

impl TryFrom<String> for Status {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(ParseError::new(value)),
        }
    }
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);

        let err = Status::try_from("WHAT".to_string()).unwrap_err();
        assert_eq!(err.to_string(),"WHAT not a valid Status string".to_string());
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
