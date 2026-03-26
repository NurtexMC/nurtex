#[derive(Debug, Clone)]
pub enum BotEvent {
  LoginFinished,
  ConfigurationFinished,
  Spawn,
  Death,
  Disconnect,
}
