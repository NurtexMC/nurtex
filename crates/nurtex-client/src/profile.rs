use uuid::Uuid;

use crate::structs::ClientInfo;

/// Структура профиля клиента
#[derive(Debug, Clone, PartialEq)]
pub struct ClientProfile {
  pub username: String,
  pub uuid: Uuid,
  pub information: ClientInfo,
  pub protocol_version: i32,
}

impl ClientProfile {
  /// Метод создания нового профиля
  pub fn new(username: String, protocol_version: i32) -> Self {
    Self {
      username: username,
      uuid: Uuid::nil(),
      information: ClientInfo::default(),
      protocol_version: protocol_version,
    }
  }
}
