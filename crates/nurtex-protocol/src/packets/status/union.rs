use std::io::{self, Cursor, Write};

use crate::packets::status::{ClientsidePongResponse, ClientsideStatusResponse, ServersidePingRequest, ServersideStatusRequest};
use crate::{IntoPacket, Packet};

#[derive(Clone, Debug, PartialEq)]
pub enum ClientsideStatusPacket {
  StatusResponse(ClientsideStatusResponse),
  PongResponse(ClientsidePongResponse),
}

impl Packet for ClientsideStatusPacket {
  fn id(&self) -> u32 {
    match self {
      Self::StatusResponse(_) => 0x0,
      Self::PongResponse(_) => 0x1,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0 => Some(Self::StatusResponse(ClientsideStatusResponse::read(buf)?)),
      1 => Some(Self::PongResponse(ClientsidePongResponse::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::StatusResponse(p) => p.write(buf),
      Self::PongResponse(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ClientsideStatusPacket> for ClientsideStatusResponse {
  fn sample(self) -> ClientsideStatusPacket {
    ClientsideStatusPacket::StatusResponse(self)
  }
}

impl IntoPacket<ClientsideStatusPacket> for ClientsidePongResponse {
  fn sample(self) -> ClientsideStatusPacket {
    ClientsideStatusPacket::PongResponse(self)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServersideStatusPacket {
  StatusRequest(ServersideStatusRequest),
  PingRequest(ServersidePingRequest),
}

impl Packet for ServersideStatusPacket {
  fn id(&self) -> u32 {
    match self {
      Self::StatusRequest(_) => 0,
      Self::PingRequest(_) => 1,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0 => Some(Self::StatusRequest(ServersideStatusRequest::read(buf)?)),
      1 => Some(Self::PingRequest(ServersidePingRequest::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::StatusRequest(p) => p.write(buf),
      Self::PingRequest(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ServersideStatusPacket> for ServersideStatusRequest {
  fn sample(self) -> ServersideStatusPacket {
    ServersideStatusPacket::StatusRequest(self)
  }
}

impl IntoPacket<ServersideStatusPacket> for ServersidePingRequest {
  fn sample(self) -> ServersideStatusPacket {
    ServersideStatusPacket::PingRequest(self)
  }
}
