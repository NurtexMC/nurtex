use std::io::{self, Cursor, Write};

use crate::packets::handshake::ServersideGreet;
use crate::{IntoPacket, Packet};

#[derive(Clone, Debug, PartialEq)]
pub enum ServersideHandshakePacket {
  Intention(ServersideGreet),
}

impl Packet for ServersideHandshakePacket {
  fn id(&self) -> u32 {
    match self {
      Self::Intention(_) => 0x0,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    if id == 0 { Some(Self::Intention(ServersideGreet::read(buf)?)) } else { None }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::Intention(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ServersideHandshakePacket> for ServersideGreet {
  fn sample(self) -> ServersideHandshakePacket {
    ServersideHandshakePacket::Intention(self)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClientsideHandshakePacket {}

impl Packet for ClientsideHandshakePacket {
  fn id(&self) -> u32 {
    0
  }

  fn read(_id: u32, _buf: &mut Cursor<&[u8]>) -> Option<Self> {
    None
  }

  fn write(&self, _buf: &mut impl Write) -> io::Result<()> {
    Ok(())
  }
}
