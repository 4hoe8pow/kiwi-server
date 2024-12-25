use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.len() > 30 {
            return Err("Player name exceeds the maximum length of 30 characters.".to_string());
        }

        if !name.chars().all(|c| c.is_alphanumeric()) {
            return Err("Player name must contain only alphanumeric characters.".to_string());
        }

        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = "ValidName123".to_string();
        let player_name = PlayerName::new(name.clone());
        assert!(player_name.is_ok());
        assert_eq!(player_name.unwrap().value(), name);
    }

    #[test]
    fn test_name_too_long() {
        let name = "a".repeat(31);
        let player_name = PlayerName::new(name);
        assert!(player_name.is_err());
        assert_eq!(
            player_name.unwrap_err(),
            "Player name exceeds the maximum length of 30 characters.".to_string()
        );
    }

    #[test]
    fn test_name_contains_non_alphanumeric() {
        let name = "Invalid@Name!".to_string();
        let player_name = PlayerName::new(name);
        assert!(player_name.is_err());
        assert_eq!(
            player_name.unwrap_err(),
            "Player name must contain only alphanumeric characters.".to_string()
        );
    }

    #[test]
    fn test_empty_name() {
        let name = "".to_string();
        let player_name = PlayerName::new(name.clone());
        assert!(player_name.is_ok()); // Assuming empty strings are valid; change if otherwise
        assert_eq!(player_name.unwrap().value(), name);
    }

    #[test]
    fn test_display_trait() {
        let name = "DisplayName".to_string();
        let player_name = PlayerName::new(name.clone()).unwrap();
        assert_eq!(format!("{}", player_name), name);
    }
}
