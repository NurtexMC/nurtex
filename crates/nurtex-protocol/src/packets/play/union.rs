use std::io::{self, Cursor, Write};

use crate::packets::play::{
  ClientsidePing, ClientsidePingResponse, ClientsideSyncPlayerPosition, MultisideKeepAlive, ServersideAcceptTeleportation, ServersidePingRequest, ServersidePong,
};
use crate::{IntoPacket, Packet};

#[derive(Clone, Debug, PartialEq)]
pub enum ClientsidePlayPacket {
  KeepAlive(MultisideKeepAlive),
  Ping(ClientsidePing),
  PingResponse(ClientsidePingResponse),
  SyncPlayerPosition(ClientsideSyncPlayerPosition),
}

impl Packet for ClientsidePlayPacket {
  fn id(&self) -> u32 {
    match self {
      Self::KeepAlive(_) => 0x2B,
      Self::Ping(_) => 0x3B,
      Self::PingResponse(_) => 0x3C,
      Self::SyncPlayerPosition(_) => 0x46,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x2B => Some(Self::KeepAlive(MultisideKeepAlive::read(buf)?)),
      0x3B => Some(Self::Ping(ClientsidePing::read(buf)?)),
      0x3C => Some(Self::PingResponse(ClientsidePingResponse::read(buf)?)),
      0x46 => Some(Self::SyncPlayerPosition(ClientsideSyncPlayerPosition::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::KeepAlive(p) => p.write(buf),
      Self::Ping(p) => p.write(buf),
      Self::PingResponse(p) => p.write(buf),
      Self::SyncPlayerPosition(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ClientsidePlayPacket> for MultisideKeepAlive {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::KeepAlive(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsidePing {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::Ping(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsidePingResponse {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::PingResponse(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsideSyncPlayerPosition {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::SyncPlayerPosition(self)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServersidePlayPacket {
  KeepAlive(MultisideKeepAlive),
  Pong(ServersidePong),
  PingRequest(ServersidePingRequest),
  AcceptTeleportation(ServersideAcceptTeleportation),
}

impl Packet for ServersidePlayPacket {
  fn id(&self) -> u32 {
    match self {
      Self::KeepAlive(_) => 0x1B,
      Self::Pong(_) => 0x2C,
      Self::PingRequest(_) => 0x25,
      Self::AcceptTeleportation(_) => 0x00,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x1B => Some(Self::KeepAlive(MultisideKeepAlive::read(buf)?)),
      0x2C => Some(Self::Pong(ServersidePong::read(buf)?)),
      0x25 => Some(Self::PingRequest(ServersidePingRequest::read(buf)?)),
      0x00 => Some(Self::AcceptTeleportation(ServersideAcceptTeleportation::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::KeepAlive(p) => p.write(buf),
      Self::Pong(p) => p.write(buf),
      Self::PingRequest(p) => p.write(buf),
      Self::AcceptTeleportation(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ServersidePlayPacket> for MultisideKeepAlive {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::KeepAlive(self)
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersidePong {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::Pong(self)
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersidePingRequest {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::PingRequest(self)
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersideAcceptTeleportation {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::AcceptTeleportation(self)
  }
}
impl IntoPacket<ClientsidePlayPacket> for ClientsidePlayPacket {
  fn sample(self) -> ClientsidePlayPacket {
    self
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersidePlayPacket {
  fn sample(self) -> ServersidePlayPacket {
    self
  }
}
