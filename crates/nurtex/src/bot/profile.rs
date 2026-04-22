use uuid::Uuid;

use crate::bot::ClientInfo;

/// Структура профиля бота
#[derive(Debug, Clone, PartialEq)]
pub struct BotProfile {
  pub username: String,
  pub uuid: Uuid,
  pub information: ClientInfo,
  pub protocol_version: i32,
}

impl BotProfile {
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
