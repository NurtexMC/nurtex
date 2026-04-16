use std::io::{self, Cursor, Write};

use crate::packets::login::{
  ClientsideCookieRequest, ClientsideEncryptionRequest, ClientsideLoginDisconnect, ClientsideLoginSuccess, ClientsidePluginRequest, ClientsideSetCompression,
  ServersideCookieResponse, ServersideEncryptionResponse, ServersideLoginAcknowledged, ServersideLoginStart, ServersidePluginResponse,
};
use crate::{IntoPacket, Packet};

#[derive(Clone, Debug, PartialEq)]
pub enum ClientsideLoginPacket {
  Disconnect(ClientsideLoginDisconnect),
  EncryptionRequest(ClientsideEncryptionRequest),
  LoginSuccess(ClientsideLoginSuccess),
  Compression(ClientsideSetCompression),
  PluginRequest(ClientsidePluginRequest),
  CookieRequest(ClientsideCookieRequest),
}

impl Packet for ClientsideLoginPacket {
  fn id(&self) -> u32 {
    match self {
      Self::Disconnect(_) => 0x00,
      Self::EncryptionRequest(_) => 0x01,
      Self::LoginSuccess(_) => 0x02,
      Self::Compression(_) => 0x03,
      Self::PluginRequest(_) => 0x04,
      Self::CookieRequest(_) => 0x05,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x00 => Some(Self::Disconnect(ClientsideLoginDisconnect::read(buf)?)),
      0x01 => Some(Self::EncryptionRequest(ClientsideEncryptionRequest::read(buf)?)),
      0x02 => Some(Self::LoginSuccess(ClientsideLoginSuccess::read(buf)?)),
      0x03 => Some(Self::Compression(ClientsideSetCompression::read(buf)?)),
      0x04 => Some(Self::PluginRequest(ClientsidePluginRequest::read(buf)?)),
      0x05 => Some(Self::CookieRequest(ClientsideCookieRequest::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::Disconnect(p) => p.write(buf),
      Self::EncryptionRequest(p) => p.write(buf),
      Self::LoginSuccess(p) => p.write(buf),
      Self::Compression(p) => p.write(buf),
      Self::PluginRequest(p) => p.write(buf),
      Self::CookieRequest(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsideLoginDisconnect {
  fn sample(self) -> ClientsideLoginPacket {
    ClientsideLoginPacket::Disconnect(self)
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsideEncryptionRequest {
  fn sample(self) -> ClientsideLoginPacket {
    ClientsideLoginPacket::EncryptionRequest(self)
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsideLoginSuccess {
  fn sample(self) -> ClientsideLoginPacket {
    ClientsideLoginPacket::LoginSuccess(self)
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsideSetCompression {
  fn sample(self) -> ClientsideLoginPacket {
    ClientsideLoginPacket::Compression(self)
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsidePluginRequest {
  fn sample(self) -> ClientsideLoginPacket {
    ClientsideLoginPacket::PluginRequest(self)
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsideCookieRequest {
  fn sample(self) -> ClientsideLoginPacket {
    ClientsideLoginPacket::CookieRequest(self)
  }
}

impl IntoPacket<ClientsideLoginPacket> for ClientsideLoginPacket {
  fn sample(self) -> ClientsideLoginPacket {
    self
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServersideLoginPacket {
  LoginStart(ServersideLoginStart),
  EncryptionResponse(ServersideEncryptionResponse),
  PluginResponse(ServersidePluginResponse),
  LoginAcknowledged(ServersideLoginAcknowledged),
  CookieResponse(ServersideCookieResponse),
}

impl Packet for ServersideLoginPacket {
  fn id(&self) -> u32 {
    match self {
      Self::LoginStart(_) => 0x00,
      Self::EncryptionResponse(_) => 0x01,
      Self::PluginResponse(_) => 0x02,
      Self::LoginAcknowledged(_) => 0x03,
      Self::CookieResponse(_) => 0x04,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x00 => Some(Self::LoginStart(ServersideLoginStart::read(buf)?)),
      0x01 => Some(Self::EncryptionResponse(ServersideEncryptionResponse::read(buf)?)),
      0x02 => Some(Self::PluginResponse(ServersidePluginResponse::read(buf)?)),
      0x03 => Some(Self::LoginAcknowledged(ServersideLoginAcknowledged::read(buf)?)),
      0x04 => Some(Self::CookieResponse(ServersideCookieResponse::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::LoginStart(p) => p.write(buf),
      Self::EncryptionResponse(p) => p.write(buf),
      Self::PluginResponse(p) => p.write(buf),
      Self::LoginAcknowledged(p) => p.write(buf),
      Self::CookieResponse(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ServersideLoginPacket> for ServersideLoginStart {
  fn sample(self) -> ServersideLoginPacket {
    ServersideLoginPacket::LoginStart(self)
  }
}

impl IntoPacket<ServersideLoginPacket> for ServersideEncryptionResponse {
  fn sample(self) -> ServersideLoginPacket {
    ServersideLoginPacket::EncryptionResponse(self)
  }
}

impl IntoPacket<ServersideLoginPacket> for ServersidePluginResponse {
  fn sample(self) -> ServersideLoginPacket {
    ServersideLoginPacket::PluginResponse(self)
  }
}

impl IntoPacket<ServersideLoginPacket> for ServersideLoginAcknowledged {
  fn sample(self) -> ServersideLoginPacket {
    ServersideLoginPacket::LoginAcknowledged(self)
  }
}

impl IntoPacket<ServersideLoginPacket> for ServersideCookieResponse {
  fn sample(self) -> ServersideLoginPacket {
    ServersideLoginPacket::CookieResponse(self)
  }
}

impl IntoPacket<ServersideLoginPacket> for ServersideLoginPacket {
  fn sample(self) -> ServersideLoginPacket {
    self
  }
}
