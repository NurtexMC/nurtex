use std::fmt::Debug;
use std::io::{self, Cursor};

use nurtex_encrypt::{AesDecryptor, AesEncryptor};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::connection::address::NurtexAddr;
use crate::packets::{
  configuration::{ClientsideConfigurationPacket, ServersideConfigurationPacket},
  handshake::{ClientsideHandshakePacket, ServersideHandshakePacket},
  login::{ClientsideLoginPacket, ServersideLoginPacket},
  play::{ClientsidePlayPacket, ServersidePlayPacket},
  status::{ClientsideStatusPacket, ServersideStatusPacket},
};
use crate::reader::{deserialize_packet, read_raw_packet};
use crate::writer::{serialize_packet, write_raw_packet};

/// Состояние подключения
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
  Handshake,
  Status,
  Login,
  Configuration,
  Play,
}

/// Универсальное перечисление `Clientside` пакетов
#[derive(Debug, Clone)]
pub enum ClientsidePacket {
  Handshake(ClientsideHandshakePacket),
  Status(ClientsideStatusPacket),
  Login(ClientsideLoginPacket),
  Configuration(ClientsideConfigurationPacket),
  Play(ClientsidePlayPacket),
}

/// Универсальное перечисление `Serverside` пакетов
#[derive(Debug, Clone)]
pub enum ServersidePacket {
  Handshake(ServersideHandshakePacket),
  Status(ServersideStatusPacket),
  Login(ServersideLoginPacket),
  Configuration(ServersideConfigurationPacket),
  Play(ServersidePlayPacket),
}

impl ServersidePacket {
  /// Вспомогательный метод создания `handshake` пакета
  pub fn handshake(packet: ServersideHandshakePacket) -> Self {
    ServersidePacket::Handshake(packet)
  }

  /// Вспомогательный метод создания `status` пакета
  pub fn status(packet: ServersideStatusPacket) -> Self {
    ServersidePacket::Status(packet)
  }

  //// Вспомогательный метод создания `login` пакета
  pub fn login(packet: ServersideLoginPacket) -> Self {
    ServersidePacket::Login(packet)
  }

  /// Вспомогательный метод создания `configuration` пакета
  pub fn configuration(packet: ServersideConfigurationPacket) -> Self {
    ServersidePacket::Configuration(packet)
  }

  /// Вспомогательный метод создания `play` пакета
  pub fn play(packet: ServersidePlayPacket) -> Self {
    ServersidePacket::Play(packet)
  }
}

/// Структура подключения
pub struct NurtexConnection {
  read_stream: OwnedReadHalf,
  write_stream: OwnedWriteHalf,
  buffer: Cursor<Vec<u8>>,
  compression_threshold: Option<u32>,
  encryptor: Option<AesEncryptor>,
  decryptor: Option<AesDecryptor>,
  state: ConnectionState,
}

impl NurtexConnection {
  /// Метод создания нового подключения
  pub async fn new(address: &NurtexAddr) -> io::Result<Self> {
    let stream = TcpStream::connect(address.unpack()).await?;
    stream.set_nodelay(true)?;
    Self::new_from_stream(stream).await
  }

  /// Метод создания нового подключения из TcpStream
  pub async fn new_from_stream(stream: TcpStream) -> io::Result<Self> {
    let (read_stream, write_stream) = stream.into_split();

    Ok(NurtexConnection {
      read_stream,
      write_stream,
      buffer: Cursor::new(Vec::new()),
      compression_threshold: None,
      encryptor: None,
      decryptor: None,
      state: ConnectionState::Handshake,
    })
  }

  /// Метод получения текущего состояния подключения
  pub fn get_state(&self) -> ConnectionState {
    self.state
  }

  /// Метод изменения состояния подключения
  pub fn set_state(&mut self, state: ConnectionState) {
    self.state = state;
  }

  /// Метод чтения пакета
  pub async fn read_packet(&mut self) -> Option<ClientsidePacket> {
    let raw_packet = read_raw_packet(&mut self.read_stream, &mut self.buffer, self.compression_threshold, &mut self.decryptor).await?;

    let mut cursor = Cursor::new(raw_packet.as_ref());

    match self.state {
      ConnectionState::Handshake => deserialize_packet::<ClientsideHandshakePacket>(&mut cursor).map(ClientsidePacket::Handshake),
      ConnectionState::Status => deserialize_packet::<ClientsideStatusPacket>(&mut cursor).map(ClientsidePacket::Status),
      ConnectionState::Login => deserialize_packet::<ClientsideLoginPacket>(&mut cursor).map(ClientsidePacket::Login),
      ConnectionState::Configuration => deserialize_packet::<ClientsideConfigurationPacket>(&mut cursor).map(ClientsidePacket::Configuration),
      ConnectionState::Play => deserialize_packet::<ClientsidePlayPacket>(&mut cursor).map(ClientsidePacket::Play),
    }
  }

  /// Вспомогательный метод чтения `status` пакета
  pub async fn read_status_packet(&mut self) -> Option<ClientsideStatusPacket> {
    let raw_packet = read_raw_packet(&mut self.read_stream, &mut self.buffer, self.compression_threshold, &mut self.decryptor).await?;

    let mut cursor = Cursor::new(raw_packet.as_ref());

    deserialize_packet::<ClientsideStatusPacket>(&mut cursor)
  }

  /// Вспомогательный метод чтения `status` пакета
  pub async fn read_login_packet(&mut self) -> Option<ClientsideLoginPacket> {
    let raw_packet = read_raw_packet(&mut self.read_stream, &mut self.buffer, self.compression_threshold, &mut self.decryptor).await?;

    let mut cursor = Cursor::new(raw_packet.as_ref());

    deserialize_packet::<ClientsideLoginPacket>(&mut cursor)
  }

  /// Вспомогательный метод чтения `configuration` пакета
  pub async fn read_configuration_packet(&mut self) -> Option<ClientsideConfigurationPacket> {
    let raw_packet = read_raw_packet(&mut self.read_stream, &mut self.buffer, self.compression_threshold, &mut self.decryptor).await?;

    let mut cursor = Cursor::new(raw_packet.as_ref());

    deserialize_packet::<ClientsideConfigurationPacket>(&mut cursor)
  }

  /// Вспомогательный метод чтения `play` пакета
  pub async fn read_play_packet(&mut self) -> Option<ClientsidePlayPacket> {
    let raw_packet = read_raw_packet(&mut self.read_stream, &mut self.buffer, self.compression_threshold, &mut self.decryptor).await?;

    let mut cursor = Cursor::new(raw_packet.as_ref());

    deserialize_packet::<ClientsidePlayPacket>(&mut cursor)
  }

  /// Метод записи пакета
  pub async fn write_packet(&mut self, packet: ServersidePacket) -> io::Result<()> {
    let serialized = match packet {
      ServersidePacket::Handshake(p) => serialize_packet(&p),
      ServersidePacket::Status(p) => serialize_packet(&p),
      ServersidePacket::Login(p) => serialize_packet(&p),
      ServersidePacket::Configuration(p) => serialize_packet(&p),
      ServersidePacket::Play(p) => serialize_packet(&p),
    }
    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to serialize packet"))?;

    write_raw_packet(&serialized, &mut self.write_stream, self.compression_threshold, &mut self.encryptor).await
  }

  /// Вспомогательный метод отправки `handshake` пакета
  pub async fn write_handshake_packet(&mut self, packet: ServersideHandshakePacket) -> io::Result<()> {
    self.write_packet(ServersidePacket::Handshake(packet)).await
  }

  /// Вспомогательный метод отправки `status` пакета
  pub async fn write_status_packet(&mut self, packet: ServersideStatusPacket) -> io::Result<()> {
    self.write_packet(ServersidePacket::Status(packet)).await
  }

  /// Вспомогательный метод отправки `login` пакета
  pub async fn write_login_packet(&mut self, packet: ServersideLoginPacket) -> io::Result<()> {
    self.write_packet(ServersidePacket::Login(packet)).await
  }

  /// Вспомогательный метод отправки `configuration` пакета
  pub async fn write_configuration_packet(&mut self, packet: ServersideConfigurationPacket) -> io::Result<()> {
    self.write_packet(ServersidePacket::Configuration(packet)).await
  }

  /// Вспомогательный метод отправки `play` пакета
  pub async fn write_play_packet(&mut self, packet: ServersidePlayPacket) -> io::Result<()> {
    self.write_packet(ServersidePacket::Play(packet)).await
  }

  /// Метод выключения потока записи
  pub async fn shutdown(&mut self) -> io::Result<()> {
    self.write_stream.shutdown().await
  }

  /// Метод установки порога сжатия
  pub fn set_compression_threshold(&mut self, threshold: i32) {
    if threshold >= 0 {
      self.compression_threshold = Some(threshold as u32);
    } else {
      self.compression_threshold = None;
    }
  }

  /// Устанавливает шифрование на соединении используя секретный ключ.
  /// Этот метод должен быть вызван **после** отправки `EncryptionResponse` серверу
  pub fn set_encryption_key(&mut self, secret_key: [u8; 16]) {
    let (encryptor, decryptor) = nurtex_encrypt::create_cipher(&secret_key);
    self.encryptor = Some(encryptor);
    self.decryptor = Some(decryptor);
  }
}
