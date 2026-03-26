use std::io::{self, Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};

use azalea_core::direction::Direction;
use azalea_core::position::{BlockPos, Vec3};
use azalea_entity::LookDirection;
use azalea_protocol::common::movements::MoveFlags;
use azalea_protocol::packets::game::s_chat::LastSeenMessagesUpdate;
use azalea_protocol::packets::game::s_player_action::Action;
use azalea_protocol::packets::game::{
  ClientboundGamePacket, ServerboundAcceptTeleportation, ServerboundChat, ServerboundClientCommand, ServerboundGamePacket, ServerboundKeepAlive, ServerboundMovePlayerPos, ServerboundMovePlayerRot, ServerboundPlayerAction, ServerboundPong, ServerboundSwing, ServerboundUseItem
};

use crate::core::bot::Bot;
use crate::core::events::BotEvent;
use crate::core::terminal::Command;

/// Дефолтный обработчик пакетов.
pub fn default_packet_processor(
  bot: &mut Bot,
  packet: ClientboundGamePacket,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = io::Result<bool>> + Send + '_>> {
  Box::pin(process_packet(bot, packet))
}

/// Дефолтный обработчик команд.
pub fn default_command_processor(
  bot: &mut Bot,
  command: Command,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = io::Result<bool>> + Send + '_>> {
  Box::pin(process_command(bot, command))
}

/// Функция обработки пакета (в состоянии Play).
async fn process_packet(bot: &mut Bot, packet: ClientboundGamePacket) -> io::Result<bool> {
  let Some(conn) = &mut bot.connection else {
    return Ok(true);
  };

  match packet {
    ClientboundGamePacket::Login(p) => {
      bot.entity_id = Some(p.player_id.0);

      if p.show_death_screen && bot.plugins.auto_respawn.enabled {
        conn
          .write(ServerboundGamePacket::ClientCommand(
            ServerboundClientCommand {
              action: azalea_protocol::packets::game::s_client_command::Action::PerformRespawn,
            },
          ))
          .await?;
      }

      bot.emit_event(BotEvent::Spawn);
    }
    ClientboundGamePacket::SetHealth(p) => {
      let state = &mut bot.components.state;
      state.health = p.health;
      state.satiety = p.food;
      state.saturation = p.saturation;
    }
    ClientboundGamePacket::MoveEntityPos(p) => {
      if !bot.is_this_my_entity_id(p.entity_id.0) {
        return Ok(true);
      }

      let physics = &mut bot.components.physics;

      physics.on_ground = p.on_ground;
    }
    ClientboundGamePacket::PlayerRotation(p) => {
      bot.components.physics.look_direction = LookDirection::new(p.y_rot, p.x_rot);
    }
    ClientboundGamePacket::PlayerPosition(p) => {
      let physics = &mut bot.components.physics;

      physics.position = p.change.pos;
      physics.velocity = p.change.delta;
      physics.look_direction = p.change.look_direction;

      conn
        .write(ServerboundGamePacket::AcceptTeleportation(
          ServerboundAcceptTeleportation { id: p.id },
        ))
        .await?;
    }
    ClientboundGamePacket::SetEntityMotion(p) => {
      if !bot.is_this_my_entity_id(p.id.0) {
        return Ok(true);
      }

      let physics = &mut bot.components.physics;
      physics.velocity = p.delta.into();
    }
    ClientboundGamePacket::KeepAlive(p) => {
      conn
        .write(ServerboundGamePacket::KeepAlive(ServerboundKeepAlive {
          id: p.id,
        }))
        .await?;
    }
    ClientboundGamePacket::Ping(p) => {
      conn
        .write(ServerboundGamePacket::Pong(ServerboundPong { id: p.id }))
        .await?;
    }
    ClientboundGamePacket::PlayerCombatKill(_p) => {
      if bot.plugins.auto_respawn.enabled {
        conn
          .write(ServerboundGamePacket::ClientCommand(
            ServerboundClientCommand {
              action: azalea_protocol::packets::game::s_client_command::Action::PerformRespawn,
            },
          ))
          .await?;
      }

      bot.emit_event(BotEvent::Death);
    }
    ClientboundGamePacket::Disconnect(p) => {
      return Err(Error::new(
        ErrorKind::ConnectionAborted,
        format!("Bot was disconnected (play): {}", p.reason.to_string()),
      ));
    }
    _ => return Ok(true),
  }

  Ok(true)
}

/// Функция обработки внешней команды.
async fn process_command(bot: &mut Bot, command: Command) -> io::Result<bool> {
  let Some(conn) = &mut bot.connection else {
    return Ok(true);
  };

  match command {
    Command::Chat(message) => {
      let start = SystemTime::now();
      let duration = start.duration_since(UNIX_EPOCH);
      let timestamp = match duration {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
      };

      conn
        .write(ServerboundGamePacket::Chat(ServerboundChat {
          message: message,
          timestamp: timestamp,
          salt: 0,
          signature: None,
          last_seen_messages: LastSeenMessagesUpdate::default(),
        }))
        .await?;
    }
    Command::SetDirection { yaw, pitch } => {
      conn
        .write(ServerboundGamePacket::MovePlayerRot(
          ServerboundMovePlayerRot {
            look_direction: LookDirection::new(yaw, pitch),
            flags: MoveFlags {
              on_ground: bot.components.physics.on_ground,
              horizontal_collision: false,
            },
          },
        ))
        .await?;
    }
    Command::SetPosition { x, y, z } => {
      conn
        .write(ServerboundGamePacket::MovePlayerPos(
          ServerboundMovePlayerPos {
            pos: Vec3::new(x, y, z),
            flags: MoveFlags {
              on_ground: bot.components.physics.on_ground,
              horizontal_collision: false,
            },
          },
        ))
        .await?;
    }
    Command::SwingArm(hand) => {
      conn
        .write(ServerboundGamePacket::Swing(ServerboundSwing { hand }))
        .await?;
    }
    Command::StartUseItem(hand) => {
      let look_direction = bot.components.physics.look_direction;

      conn.write(ServerboundGamePacket::UseItem(ServerboundUseItem {
        hand: hand,
        seq: 0,
        y_rot: look_direction.y_rot(),
        x_rot: look_direction.x_rot()
      })).await?;
    }
    Command::ReleaseUseItem => {
      conn.write(ServerboundGamePacket::PlayerAction(ServerboundPlayerAction {
        action: Action::ReleaseUseItem,
        pos: BlockPos::new(0, 0, 0),
        direction: Direction::Down,
        seq: 0
      })).await?;
    }
    Command::SendPacket(packet) => {
      conn.write(packet).await?;
    }
    Command::Disconnect => {
      bot.disconnect().await?;
      return Ok(false);
    }
  }

  Ok(true)
}
