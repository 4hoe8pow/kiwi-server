use crate::domain::value_objects::room_code::RoomCode;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub room_code: RoomCode,
    pub players: Vec<Player>,
}

impl Room {
    // 満室か
    pub fn is_full(&self) -> bool {
        self.players.len() > 1 // 最大2人
    }

    // プレイヤーを追加
    pub fn add_player(&mut self, player: Player) {
        if !self.is_full() {
            self.players.push(player);
        }
    }

    // 部屋を取得
    pub fn get_players(&self) -> Vec<String> {
        self.players
            .iter()
            .map(|p| p.player_name.value().to_string())
            .collect()
    }
}

impl Room {
    pub fn new(room_code: RoomCode) -> Self {
        Self {
            room_code,
            players: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::player_name::PlayerName;
    use crate::domain::value_objects::room_code::RoomCode; // PlayerNameのインポートも必要です

    #[test]
    fn test_add_player_success() {
        // テストデータ
        let room_code = RoomCode::new("Room123".to_string()).unwrap();
        let mut room = Room::new(room_code);

        // プレイヤーを追加
        let player_name = PlayerName::new("Player1".to_string()).unwrap();
        let player = Player::new(player_name);
        room.add_player(player);

        // プレイヤーが追加されているか確認
        assert_eq!(room.get_players().len(), 1);
    }

    #[test]
    fn test_add_player_fail_when_full() {
        // テストデータ
        let room_code = RoomCode::new("Room123".to_string()).unwrap();
        let mut room = Room::new(room_code);

        // プレイヤーを2人追加
        let player_name1 = PlayerName::new("Player1".to_string()).unwrap();
        let player1 = Player::new(player_name1);
        room.add_player(player1);

        let player_name2 = PlayerName::new("Player2".to_string()).unwrap();
        let player2 = Player::new(player_name2);
        room.add_player(player2);

        // プレイヤーが2人の時にもう1人追加しようとした場合
        let player_name3 = PlayerName::new("Player3".to_string()).unwrap();
        let player3 = Player::new(player_name3);
        room.add_player(player3);

        // 2人以上のプレイヤーが追加されていないことを確認
        assert_eq!(room.get_players().len(), 2);
    }

    #[test]
    fn test_is_full_true() {
        // テストデータ
        let room_code = RoomCode::new("Room123".to_string()).unwrap();
        let mut room = Room::new(room_code);

        // プレイヤーを2人追加
        let player_name1 = PlayerName::new("Player1".to_string()).unwrap();
        let player1 = Player::new(player_name1);
        room.add_player(player1);

        let player_name2 = PlayerName::new("Player2".to_string()).unwrap();
        let player2 = Player::new(player_name2);
        room.add_player(player2);

        // 満室チェック
        assert!(room.is_full());
    }

    #[test]
    fn test_is_full_false() {
        // テストデータ
        let room_code = RoomCode::new("Room123".to_string()).unwrap();
        let mut room = Room::new(room_code);

        // プレイヤーを1人追加
        let player_name = PlayerName::new("Player1".to_string()).unwrap();
        let player = Player::new(player_name);
        room.add_player(player);

        // 満室チェック
        assert!(!room.is_full());
    }

    #[test]
    fn test_get_players() {
        // テストデータ
        let room_code = RoomCode::new("Room123".to_string()).unwrap();
        let mut room = Room::new(room_code);

        // プレイヤーを2人追加
        let player_name1 = PlayerName::new("Player1".to_string()).unwrap();
        let player1 = Player::new(player_name1);
        room.add_player(player1);

        let player_name2 = PlayerName::new("Player2".to_string()).unwrap();
        let player2 = Player::new(player_name2);
        room.add_player(player2);

        // プレイヤー名が正しく取得できるか確認
        let players = room.get_players();
        assert_eq!(players, vec!["Player1".to_string(), "Player2".to_string()]);
    }
}
