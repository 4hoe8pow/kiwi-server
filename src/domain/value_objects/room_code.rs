use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomCode(String);

impl RoomCode {
    pub fn new(code: String) -> Result<Self, String> {
        if code.len() > 10 {
            return Err("Room code exceeds the maximum length of 10 characters.".to_string());
        }

        if !code.chars().all(|c| c.is_alphanumeric()) {
            return Err("Room code must contain only alphanumeric characters.".to_string());
        }

        Ok(Self(code))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoomCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_code() {
        let code = "Room123".to_string();
        let room_code = RoomCode::new(code.clone());
        assert!(room_code.is_ok());
        assert_eq!(room_code.unwrap().value(), code);
    }

    #[test]
    fn test_code_too_long() {
        let code = "a".repeat(11);
        let room_code = RoomCode::new(code);
        assert!(room_code.is_err());
        assert_eq!(
            room_code.unwrap_err(),
            "Room code exceeds the maximum length of 10 characters.".to_string()
        );
    }

    #[test]
    fn test_code_contains_non_alphanumeric() {
        let code = "Room@123".to_string();
        let room_code = RoomCode::new(code);
        assert!(room_code.is_err());
        assert_eq!(
            room_code.unwrap_err(),
            "Room code must contain only alphanumeric characters.".to_string()
        );
    }

    #[test]
    fn test_empty_code() {
        let code = "".to_string();
        let room_code = RoomCode::new(code.clone());
        assert!(room_code.is_ok()); // Assuming empty strings are valid; change if otherwise
        assert_eq!(room_code.unwrap().value(), code);
    }

    #[test]
    fn test_display_trait() {
        let code = "Display123".to_string();
        let room_code = RoomCode::new(code.clone()).unwrap();
        assert_eq!(format!("{}", room_code), code);
    }
}
