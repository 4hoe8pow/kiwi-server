use crate::domain::{
    entities::room::Room, repositories::room_repository::RoomRepository,
    value_objects::room_code::RoomCode,
};

pub struct RoomService<R: RoomRepository> {
    pub repository: R,
}

impl<R: RoomRepository> RoomService<R> {
    pub fn new(repository: R) -> Self {
        RoomService { repository }
    }

    // 部屋コードがすでに存在するかを確認
    pub async fn exists(&self, room_code: &RoomCode) -> bool {
        self.repository
            .exists(room_code.value())
            .await
            .unwrap_or_default()
    }

    // 部屋を新規作成
    pub async fn create_room(&self, room_code: &RoomCode) -> Option<Room> {
        if self.exists(room_code).await {
            return None; // すでに存在する場合は `None` を返す
        }

        let room = Room::new(room_code.clone());

        match self.repository.save(room.clone().into()).await {
            Ok(_) => Some(room),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::room_repository::MockRoomRepository;
    use crate::domain::value_objects::room_code::RoomCode;

    // テストの準備と実行にtokioランタイムを使用
    #[tokio::test]
    async fn test_exists_room_code_found() {
        let mut mock_repo = MockRoomRepository::new();
        let room_code = RoomCode::new("Room123".to_string()).unwrap();

        mock_repo
            .expect_exists()
            .withf(|code| code == "Room123")
            .returning(|_| Ok(true));

        let service = RoomService::new(mock_repo);
        assert!(service.exists(&room_code).await);
    }

    #[tokio::test]
    async fn test_exists_room_code_not_found() {
        let mut mock_repo = MockRoomRepository::new();
        let room_code = RoomCode::new("Room123".to_string()).unwrap();

        mock_repo
            .expect_exists()
            .withf(|code| code == "Room123")
            .returning(|_| Ok(false));

        let service = RoomService::new(mock_repo);
        assert!(!service.exists(&room_code).await);
    }

    #[tokio::test]
    async fn test_create_room_success() {
        let mut mock_repo = MockRoomRepository::new();
        let room_code = RoomCode::new("Room123".to_string()).unwrap();

        mock_repo
            .expect_exists()
            .withf(|code| code == "Room123")
            .returning(|_| Ok(false));

        mock_repo.expect_save().returning(|_| Ok(()));

        let service = RoomService::new(mock_repo);
        let result = service.create_room(&room_code).await;

        assert!(result.is_some());
        assert_eq!(result.unwrap().room_code.value(), "Room123");
    }

    #[tokio::test]
    async fn test_create_room_already_exists() {
        let mut mock_repo = MockRoomRepository::new();
        let room_code = RoomCode::new("Room123".to_string()).unwrap();

        mock_repo
            .expect_exists()
            .withf(|code| code == "Room123")
            .returning(|_| Ok(true));

        let service = RoomService::new(mock_repo);
        let result = service.create_room(&room_code).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_create_room_save_error() {
        let mut mock_repo = MockRoomRepository::new();
        let room_code = RoomCode::new("Room123".to_string()).unwrap();

        mock_repo
            .expect_exists()
            .withf(|code| code == "Room123")
            .returning(|_| Ok(false));

        mock_repo
            .expect_save()
            .returning(|_| Err("Save error".to_string()));

        let service = RoomService::new(mock_repo);
        let result = service.create_room(&room_code).await;

        assert!(result.is_none());
    }
}
