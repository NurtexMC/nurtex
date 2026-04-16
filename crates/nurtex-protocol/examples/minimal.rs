use std::io;

use nurtex_protocol::connection::NurtexConnection;
use nurtex_protocol::connection::address::convert_address;
use nurtex_protocol::connection::utils::handle_encryption_request;
use nurtex_protocol::packets::configuration::{
  ClientsideConfigurationPacket, ResourcePackState, ServersideAcknowledgeFinishConfiguration, ServersideClientInformation, ServersideConfigurationPacket, ServersideKnownPacks,
  ServersideResourcePackResponse,
};
use nurtex_protocol::packets::handshake::{ClientIntention, ClientsideHandshakePacket, ServersideGreet, ServersideHandshakePacket};
use nurtex_protocol::packets::login::{ClientsideLoginPacket, ServersideLoginAcknowledged, ServersideLoginPacket, ServersideLoginStart};
use nurtex_protocol::packets::play::{ClientsidePlayPacket, ServersidePlayPacket};

#[tokio::main]
async fn main() -> io::Result<()> {
  // Конвертируем адрес сервера
  let addr = convert_address("localhost:25565").unwrap();

  // Создаём подключение (состояние Handshake)
  let mut conn: NurtexConnection<ClientsideHandshakePacket, ServersideHandshakePacket> = match NurtexConnection::new(&addr).await {
    Ok(c) => c,
    Err(_) => return Ok(()),
  };

  // Отправляем привестствие
  conn
    .write_packet(ServersideGreet {
      protocol_version: 774, // Версия 1.21.11
      server_host: addr.ip().to_string(),
      server_port: addr.port(),
      intention: ClientIntention::Login,
    })
    .await?;

  // Меняем состояние подключения на Login
  let mut conn = conn.change_state::<ClientsideLoginPacket, ServersideLoginPacket>();

  // Отправляем пакет LoginStart где указываем имя клиента и UUID (для оффлайн серверов просто нулевой)
  conn
    .write_packet(ServersideLoginStart {
      username: "NurtexBot".to_string(),
      uuid: uuid::Uuid::nil(),
    })
    .await?;

  // Создаём цикл для обработки Clientside пакетов в состоянии Login
  loop {
    if let Some(p) = conn.read_packet().await {
      match p {
        ClientsideLoginPacket::Compression(p) => {
          // Устанавливаем порог сжатия
          conn.set_compression_threshold(p.compression_threshold);
        }
        ClientsideLoginPacket::EncryptionRequest(request) => {
          // Пробуем обработать запрос шифрования
          if let Some((response, secret_key)) = handle_encryption_request(&request) {
            conn.write_packet(response).await?;
            conn.set_encryption_key(secret_key);
          }
        }
        ClientsideLoginPacket::LoginSuccess(_p) => {
          // Всё, логин пройден, отправляем LoginAcknowledged и выходим из цикла
          conn.write_packet(ServersideLoginAcknowledged).await?;
          break;
        }
        _ => {}
      }
    } else {
      break;
    }
  }

  // Меняем состояние подключения на Configuration
  let mut conn = conn.change_state::<ClientsideConfigurationPacket, ServersideConfigurationPacket>();

  // Отправляем опции клиента
  conn
    .write_packet(ServersideClientInformation {
      locale: "en_US".to_string(),
      view_distance: 10,
      chat_mode: 0,
      chat_colors: true,
      displayed_skin_parts: 0x7F,
      main_hand: 1,
      enable_text_filtering: false,
      allow_server_listings: true,
      particle_status: 0,
    })
    .await?;

  // Создаём цикл для обработки Clientside пакетов в состоянии Configuration
  loop {
    if let Some(p) = conn.read_packet().await {
      match p {
        ClientsideConfigurationPacket::KeepAlive(p) => {
          // Отправляем ответ на KeepAlive
          conn.write_packet(nurtex_protocol::packets::configuration::MultisideKeepAlive { id: p.id }).await?;
        }
        ClientsideConfigurationPacket::Ping(p) => {
          // Отправляем ответ на Ping
          conn.write_packet(nurtex_protocol::packets::configuration::ServersidePong { id: p.id }).await?;
        }
        ClientsideConfigurationPacket::KnownPacks(p) => {
          // Отправляем паки
          conn.write_packet(ServersideKnownPacks { known_packs: p.known_packs }).await?;
        }
        ClientsideConfigurationPacket::FinishConfiguration(_) => {
          // Всё, конфигурация пройдена, отправляем AcknowledgeFinishConfiguration и выходим из цикла
          conn.write_packet(ServersideAcknowledgeFinishConfiguration).await?;
          break;
        }
        ClientsideConfigurationPacket::AddResourcePack(p) => {
          // Принимаем ресурс пак
          conn
            .write_packet(ServersideResourcePackResponse {
              uuid: p.uuid,
              state: ResourcePackState::Accepted,
            })
            .await?;
        }
        _ => {}
      }
    } else {
      break;
    }
  }

  // Меняем состояние подключения на Play
  let mut conn = conn.change_state::<ClientsidePlayPacket, ServersidePlayPacket>();

  // Создаём цикл обработки пакетов в состоянии Play
  loop {
    if let Some(p) = conn.read_packet().await {
      match p {
        ClientsidePlayPacket::KeepAlive(p) => {
          // Отправляем ответ на KeepAlive
          conn.write_packet(nurtex_protocol::packets::play::MultisideKeepAlive { id: p.id }).await?;
        }
        ClientsidePlayPacket::Ping(p) => {
          // Отправляем ответ на Ping
          conn.write_packet(nurtex_protocol::packets::play::ServersidePong { id: p.id }).await?;
        }
        _ => {}
      }
    }
  }
}
