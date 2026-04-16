use std::fmt::Debug;
use std::io::{self, Cursor};
use std::marker::PhantomData;
use std::net::SocketAddr;

use nurtex_encrypt::{AesDecryptor, AesEncryptor};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use crate::reader::{deserialize_packet, read_raw_packet};
use crate::writer::{serialize_packet, write_raw_packet};
use crate::{IntoPacket, Packet};

pub struct NurtexConnection<RS: Packet, WS: Packet> {
  read_stream: OwnedReadHalf,
  write_stream: OwnedWriteHalf,
  buffer: Cursor<Vec<u8>>,
  compression_threshold: Option<u32>,
  encryptor: Option<AesEncryptor>,
  decryptor: Option<AesDecryptor>,
  _phantom_reader: PhantomData<RS>,
  _phantom_writer: PhantomData<WS>,
}

impl<RS, WS> NurtexConnection<RS, WS>
where
  RS: Packet + Debug,
  WS: Packet + Debug,
{
  /// Метод создания нового подключения
  pub async fn new(address: &SocketAddr) -> io::Result<Self> {
    let stream = TcpStream::connect(address).await?;
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
      _phantom_reader: PhantomData,
      _phantom_writer: PhantomData,
    })
  }

  /// Метод чтения пакета
  pub async fn read_packet(&mut self) -> Option<RS> {
    let raw_packet = read_raw_packet(&mut self.read_stream, &mut self.buffer, self.compression_threshold, &mut self.decryptor).await?;
    deserialize_packet(&mut Cursor::new(&raw_packet))
  }

  /// Метод записи пакета
  pub async fn write_packet(&mut self, packet: impl IntoPacket<WS>) -> io::Result<()> {
    let packet = packet.sample();
    let serialized = serialize_packet(&packet).unwrap();
    write_raw_packet(&serialized, &mut self.write_stream, self.compression_threshold, &mut self.encryptor).await
  }

  /// Метод выключения потока записи
  pub async fn shutdown(&mut self) -> io::Result<()> {
    self.write_stream.shutdown().await
  }

  /// Метод изменения состояния подключения
  pub fn change_state<RS1, WS1>(self) -> NurtexConnection<RS1, WS1>
  where
    RS1: Packet + Debug,
    WS1: Packet + Debug,
  {
    NurtexConnection {
      read_stream: self.read_stream,
      write_stream: self.write_stream,
      buffer: self.buffer,
      compression_threshold: self.compression_threshold,
      encryptor: self.encryptor,
      decryptor: self.decryptor,
      _phantom_reader: PhantomData,
      _phantom_writer: PhantomData,
    }
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
