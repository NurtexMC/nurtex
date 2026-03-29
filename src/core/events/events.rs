use azalea_protocol::packets::game::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum BotEvent {
  LoginFinished,
  ConfigurationFinished,
  Spawn,
  Death,
  Disconnect,
  Chat {
    sender_uuid: Option<Uuid>,
    message: String,
  },
  Packet(ClientboundGamePacket),
}
