#![allow(dead_code)]

use azalea_protocol::packets::game::{ServerboundGamePacket, s_interact::InteractionHand};
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub enum Command {
  Chat(String),
  Jump,
  SetDirection { yaw: f32, pitch: f32 },
  SetPosition { x: f64, y: f64, z: f64 },
  SwingArm(InteractionHand),
  SendPacket(ServerboundGamePacket),
  Disconnect,
}

#[derive(Clone)]
pub struct Terminal {
  pub username: String,
  pub sender: Sender<Command>,
}

impl Terminal {
  pub async fn send(&self, command: Command) {
    let _ = self.sender.send(command).await;
  }
}
