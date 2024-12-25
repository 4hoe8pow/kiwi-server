use serde::{Deserialize, Serialize};

use crate::domain::value_objects::player_name::PlayerName;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub player_name: PlayerName,
}

impl Player {
    pub fn new(player_name: PlayerName) -> Self {
        Self { player_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::player_name::PlayerName;

    #[test]
    fn test_create_player_success() {
        // 正しいプレイヤー名でPlayerを作成
        let player_name = PlayerName::new("Player1".to_string()).unwrap();
        let player = Player::new(player_name);

        // プレイヤー名が正しく設定されているか確認
        assert_eq!(player.player_name.value(), "Player1");
    }

    #[test]
    fn test_create_player_name_too_long() {
        // 名前が30文字を超える場合、エラーが発生することを確認
        let long_name = "A".repeat(31);
        let player_name = PlayerName::new(long_name);

        assert!(player_name.is_err());
        assert_eq!(
            player_name.unwrap_err(),
            "Player name exceeds the maximum length of 30 characters.".to_string()
        );
    }

    #[test]
    fn test_create_player_name_non_alphanumeric() {
        // 非英数字を含む名前の場合、エラーが発生することを確認
        let invalid_name = "Player@123".to_string();
        let player_name = PlayerName::new(invalid_name);

        assert!(player_name.is_err());
        assert_eq!(
            player_name.unwrap_err(),
            "Player name must contain only alphanumeric characters.".to_string()
        );
    }

    #[test]
    fn test_create_player_name_empty() {
        // 空のプレイヤー名を検証
        let empty_name = "".to_string();
        let player_name = PlayerName::new(empty_name.clone());

        assert!(player_name.is_ok()); // 空の名前が許容されている場合
        let player = Player::new(player_name.unwrap());
        assert_eq!(player.player_name.value(), empty_name);
    }
}
