use std::io::{self, Cursor, Write};

use crate::packets::play::{
  ClientsideDamageEvent, ClientsideLogin, ClientsidePing, ClientsidePingResponse, ClientsidePlayerCombatKill, ClientsidePlayerPosition, ClientsidePlayerRotation,
  ClientsideSetExperience, ClientsideSetHealth, ClientsideUpdateEntityPos, MultisideKeepAlive, ServersideAcceptTeleportation, ServersideMovePlayerPos, ServersidePingRequest,
  ServersidePong, ServersideSwingArm, ServersideUseItem,
};
use crate::{IntoPacket, Packet};

#[derive(Clone, Debug, PartialEq)]
pub enum ClientsidePlayPacket {
  KeepAlive(MultisideKeepAlive),
  Ping(ClientsidePing),
  PingResponse(ClientsidePingResponse),
  DamageEvent(ClientsideDamageEvent),
  UpdateEntityPos(ClientsideUpdateEntityPos),
  Login(ClientsideLogin),
  PlayerPosition(ClientsidePlayerPosition),
  PlayerRotation(ClientsidePlayerRotation),
  PlayerCombatKill(ClientsidePlayerCombatKill),
  SetHealth(ClientsideSetHealth),
  SetExperience(ClientsideSetExperience),
}

impl Packet for ClientsidePlayPacket {
  fn id(&self) -> u32 {
    match self {
      Self::KeepAlive(_) => 0x2B,
      Self::Ping(_) => 0x3B,
      Self::PingResponse(_) => 0x3C,
      Self::DamageEvent(_) => 0x19,
      Self::UpdateEntityPos(_) => 0x33,
      Self::Login(_) => 0x30,
      Self::PlayerPosition(_) => 0x46,
      Self::PlayerRotation(_) => 0x47,
      Self::PlayerCombatKill(_) => 0x42,
      Self::SetHealth(_) => 0x66,
      Self::SetExperience(_) => 0x65,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x2B => Some(Self::KeepAlive(MultisideKeepAlive::read(buf)?)),
      0x3B => Some(Self::Ping(ClientsidePing::read(buf)?)),
      0x3C => Some(Self::PingResponse(ClientsidePingResponse::read(buf)?)),
      0x19 => Some(Self::DamageEvent(ClientsideDamageEvent::read(buf)?)),
      0x33 => Some(Self::UpdateEntityPos(ClientsideUpdateEntityPos::read(buf)?)),
      0x30 => Some(Self::Login(ClientsideLogin::read(buf)?)),
      0x46 => Some(Self::PlayerPosition(ClientsidePlayerPosition::read(buf)?)),
      0x47 => Some(Self::PlayerRotation(ClientsidePlayerRotation::read(buf)?)),
      0x42 => Some(Self::PlayerCombatKill(ClientsidePlayerCombatKill::read(buf)?)),
      0x66 => Some(Self::SetHealth(ClientsideSetHealth::read(buf)?)),
      0x65 => Some(Self::SetExperience(ClientsideSetExperience::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::KeepAlive(p) => p.write(buf),
      Self::Ping(p) => p.write(buf),
      Self::PingResponse(p) => p.write(buf),
      Self::DamageEvent(p) => p.write(buf),
      Self::UpdateEntityPos(p) => p.write(buf),
      Self::Login(p) => p.write(buf),
      Self::PlayerPosition(p) => p.write(buf),
      Self::PlayerRotation(p) => p.write(buf),
      Self::PlayerCombatKill(p) => p.write(buf),
      Self::SetHealth(p) => p.write(buf),
      Self::SetExperience(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsidePlayPacket {
  fn sample(self) -> ClientsidePlayPacket {
    self
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

impl IntoPacket<ClientsidePlayPacket> for ClientsidePlayerPosition {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::PlayerPosition(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsideDamageEvent {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::DamageEvent(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsideUpdateEntityPos {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::UpdateEntityPos(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsideLogin {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::Login(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsidePlayerRotation {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::PlayerRotation(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsidePlayerCombatKill {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::PlayerCombatKill(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsideSetHealth {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::SetHealth(self)
  }
}

impl IntoPacket<ClientsidePlayPacket> for ClientsideSetExperience {
  fn sample(self) -> ClientsidePlayPacket {
    ClientsidePlayPacket::SetExperience(self)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServersidePlayPacket {
  KeepAlive(MultisideKeepAlive),
  Pong(ServersidePong),
  PingRequest(ServersidePingRequest),
  AcceptTeleportation(ServersideAcceptTeleportation),
  SwingArm(ServersideSwingArm),
  UseItem(ServersideUseItem),
  MovePlayerPos(ServersideMovePlayerPos),
}

impl Packet for ServersidePlayPacket {
  fn id(&self) -> u32 {
    match self {
      Self::KeepAlive(_) => 0x1B,
      Self::Pong(_) => 0x2C,
      Self::PingRequest(_) => 0x25,
      Self::AcceptTeleportation(_) => 0x00,
      Self::SwingArm(_) => 0x3C,
      Self::UseItem(_) => 0x40,
      Self::MovePlayerPos(_) => 0x1D,
    }
  }

  fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x1B => Some(Self::KeepAlive(MultisideKeepAlive::read(buf)?)),
      0x2C => Some(Self::Pong(ServersidePong::read(buf)?)),
      0x25 => Some(Self::PingRequest(ServersidePingRequest::read(buf)?)),
      0x00 => Some(Self::AcceptTeleportation(ServersideAcceptTeleportation::read(buf)?)),
      0x3C => Some(Self::SwingArm(ServersideSwingArm::read(buf)?)),
      0x40 => Some(Self::UseItem(ServersideUseItem::read(buf)?)),
      0x1D => Some(Self::MovePlayerPos(ServersideMovePlayerPos::read(buf)?)),
      _ => None,
    }
  }

  fn write(&self, buf: &mut impl Write) -> io::Result<()> {
    match self {
      Self::KeepAlive(p) => p.write(buf),
      Self::Pong(p) => p.write(buf),
      Self::PingRequest(p) => p.write(buf),
      Self::AcceptTeleportation(p) => p.write(buf),
      Self::SwingArm(p) => p.write(buf),
      Self::UseItem(p) => p.write(buf),
      Self::MovePlayerPos(p) => p.write(buf),
    }
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersidePlayPacket {
  fn sample(self) -> ServersidePlayPacket {
    self
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

impl IntoPacket<ServersidePlayPacket> for ServersideSwingArm {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::SwingArm(self)
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersideUseItem {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::UseItem(self)
  }
}

impl IntoPacket<ServersidePlayPacket> for ServersideMovePlayerPos {
  fn sample(self) -> ServersidePlayPacket {
    ServersidePlayPacket::MovePlayerPos(self)
  }
}
