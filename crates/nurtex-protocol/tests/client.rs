#[cfg(test)]
mod tests {
  use std::io;

  use nurtex_protocol::connection::address::convert_address;
  use nurtex_protocol::connection::utils::handle_encryption_request;
  use nurtex_protocol::packets::configuration::{ResourcePackState, ServersideResourcePackResponse};
  use nurtex_protocol::packets::play::{ClientsidePlayPacket, ServersidePlayPacket};
  use nurtex_protocol::{
    connection::NurtexConnection,
    packets::{
      configuration::{ClientsideConfigurationPacket, ServersideAcknowledgeFinishConfiguration, ServersideClientInformation, ServersideConfigurationPacket, ServersideKnownPacks},
      handshake::{ClientIntention, ClientsideHandshakePacket, ServersideGreet, ServersideHandshakePacket},
      login::{ClientsideLoginPacket, ServersideLoginAcknowledged, ServersideLoginPacket, ServersideLoginStart},
    },
  };

  #[tokio::test]
  async fn create_client() -> io::Result<()> {
    let addr = convert_address("localhost:25565").unwrap();

    let conn_result = NurtexConnection::new(&addr).await;

    let mut conn: NurtexConnection<ClientsideHandshakePacket, ServersideHandshakePacket> = match conn_result {
      Ok(c) => c,
      Err(_) => return Ok(()),
    };

    conn
      .write_packet(ServersideGreet {
        protocol_version: 774,
        server_host: addr.ip().to_string(),
        server_port: addr.port(),
        intention: ClientIntention::Login,
      })
      .await?;

    let mut conn = conn.change_state::<ClientsideLoginPacket, ServersideLoginPacket>();

    conn
      .write_packet(ServersideLoginStart {
        username: "NurtexBot".to_string(),
        uuid: uuid::Uuid::nil(),
      })
      .await?;

    loop {
      if let Some(p) = conn.read_packet().await {
        match p {
          ClientsideLoginPacket::Compression(p) => {
            println!("[compression_threshold] Threshold: {}", p.compression_threshold);
            conn.set_compression_threshold(p.compression_threshold);
          }
          ClientsideLoginPacket::EncryptionRequest(request) => {
            println!("[encryption] Should authenticate: {}", request.should_authenticate);

            if let Some((response, secret_key)) = handle_encryption_request(&request) {
              conn.write_packet(response).await?;
              conn.set_encryption_key(secret_key);
            }
          }
          ClientsideLoginPacket::LoginSuccess(p) => {
            println!("[login] Username: {}", p.username);
            println!("[login] UUID: {}", p.uuid);
            println!("[login] Properties: {} items", p.properties.len());

            conn.write_packet(ServersideLoginAcknowledged).await?;

            break;
          }
          _ => {}
        }
      } else {
        break;
      }
    }

    let mut conn = conn.change_state::<ClientsideConfigurationPacket, ServersideConfigurationPacket>();

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

    loop {
      if let Some(p) = conn.read_packet().await {
        match p {
          ClientsideConfigurationPacket::Disconnect(p) => {
            println!("[disconnect] Reason (bytes): {:?}", p.reason);
            break;
          }
          ClientsideConfigurationPacket::KeepAlive(p) => {
            println!("[keep_alive] ID: {}", p.id);
            conn.write_packet(nurtex_protocol::packets::configuration::MultisideKeepAlive { id: p.id }).await?;
          }
          ClientsideConfigurationPacket::Ping(p) => {
            println!("[ping] ID: {}", p.id);
            conn.write_packet(nurtex_protocol::packets::configuration::ServersidePong { id: p.id }).await?;
          }
          ClientsideConfigurationPacket::RegistryData(p) => {
            println!("[registry_data] ID: {}, Length: {} bytes)", p.registry_id, p.raw_data.len());
          }
          ClientsideConfigurationPacket::FeatureFlags(p) => {
            println!("[feature_flags] Length: {}", p.features.len());
          }
          ClientsideConfigurationPacket::UpdateTags(p) => {
            println!("[update_tags] Length: {}", p.tags.len());
          }
          ClientsideConfigurationPacket::KnownPacks(p) => {
            for pack in &p.known_packs {
              println!("[known_pack] {}:{}, Version: {}", pack.namespace, pack.id, pack.version);
            }

            conn.write_packet(ServersideKnownPacks { known_packs: p.known_packs }).await?;
          }
          ClientsideConfigurationPacket::FinishConfiguration(_) => {
            conn.write_packet(ServersideAcknowledgeFinishConfiguration).await?;
            break;
          }
          ClientsideConfigurationPacket::PluginMessage(p) => {
            println!("[plugin_message] Channel: {}, Data: {:?}", p.channel, p.data);
          }
          ClientsideConfigurationPacket::AddResourcePack(p) => {
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

    let mut conn = conn.change_state::<ClientsidePlayPacket, ServersidePlayPacket>();

    loop {
      if let Some(p) = conn.read_packet().await {
        match p {
          ClientsidePlayPacket::KeepAlive(p) => {
            println!("[keep_alive] ID: {}", p.id);
            conn.write_packet(nurtex_protocol::packets::play::MultisideKeepAlive { id: p.id }).await?;
          }
          ClientsidePlayPacket::Ping(p) => {
            println!("[ping] ID: {}", p.id);
            conn.write_packet(nurtex_protocol::packets::play::ServersidePong { id: p.id }).await?;
          }
          ClientsidePlayPacket::SyncPlayerPosition(p) => {
            println!(
              "[sync_position] Teleport ID: {}, X: {}, Y: {}, Z: {}, Yaw: {}, Pitch: {}, Flags: {:?}",
              p.teleport_id, p.position_x, p.position_y, p.position_z, p.yaw, p.pitch, p.teleport_flags
            );
            conn
              .write_packet(nurtex_protocol::packets::play::ServersideAcceptTeleportation { teleport_id: p.teleport_id })
              .await?;
          }
          _ => {}
        }
      }
    }
  }
}
