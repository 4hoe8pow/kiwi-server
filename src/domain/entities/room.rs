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
