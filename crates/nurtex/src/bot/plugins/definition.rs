use crate::bot::plugins::{AutoReconnectPlugin, AutoRespawnPlugin};

/// Структура плагинов бота
#[derive(Clone)]
pub struct BotPlugins {
  pub auto_respawn: AutoRespawnPlugin,
  pub auto_reconnect: AutoReconnectPlugin,
}

impl Default for BotPlugins {
  fn default() -> Self {
    Self {
      auto_respawn: AutoRespawnPlugin::default(),
      auto_reconnect: AutoReconnectPlugin::default(),
    }
  }
}
