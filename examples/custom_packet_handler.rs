use std::io;

use std::io::{Error, ErrorKind};
use std::sync::Arc;

use azalea_protocol::packets::game::{ClientboundGamePacket, ServerboundGamePacket, ServerboundKeepAlive, ServerboundPong};
use nurtex::bot::Bot;
use nurtex::bot::transmitter::BotPackage;
use nurtex::create_bot;

/// Создаём кастомный процессор пакетов
pub fn custom_packet_processor<P: BotPackage>(
  bot: &mut Bot<P>,
  packet: Arc<ClientboundGamePacket>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = io::Result<bool>> + Send + '_>> {
  Box::pin(process_packet(bot, packet))
}

/// Кастомная реализация функции для обработки пакета.
/// В данном примере обрабатываются только 2 **основных**
/// пакета - `KeepAlive` и `Ping`, все остальные просто игнорируются
async fn process_packet<P: BotPackage>(bot: &mut Bot<P>, packet: Arc<ClientboundGamePacket>) -> io::Result<bool> {
  match &*packet {
    ClientboundGamePacket::KeepAlive(p) => {
      println!("Получен пакет KeepAlive");

      let Some(conn) = &mut bot.connection else {
        return Err(Error::new(ErrorKind::NotConnected, "Connection could not be obtained"));
      };

      conn.write(ServerboundGamePacket::KeepAlive(ServerboundKeepAlive { id: p.id })).await?;
    }
    ClientboundGamePacket::Ping(p) => {
      println!("Получен пакет Ping");

      let Some(conn) = &mut bot.connection else {
        return Err(Error::new(ErrorKind::NotConnected, "Connection could not be obtained"));
      };

      conn.write(ServerboundGamePacket::Pong(ServerboundPong { id: p.id })).await?;
    }
    _ => return Ok(true),
  }

  Ok(true)
}

#[tokio::main]
async fn main() -> io::Result<()> {
  // Создаём бота
  let bot = create_bot("NurtexBot");

  bot
    .set_packet_processor(custom_packet_processor) // Задаём кастомный процессор пакетов
    .connect_to("localhost", 25565)
    .await
}
