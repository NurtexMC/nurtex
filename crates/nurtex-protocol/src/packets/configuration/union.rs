use std::io::{self, Cursor, Write};

use crate::packets::configuration::{
  ClientsideAddResourcePack, ClientsideCookieRequest, ClientsideCustomReportDetails, ClientsideDisconnect, ClientsideFeatureFlags, ClientsideFinishConfiguration,
  ClientsideKnownPacks, ClientsidePing, ClientsidePluginMessage, ClientsideRegistryData, ClientsideRemoveResourcePack, ClientsideResetChat, ClientsideServerLinks,
  ClientsideStoreCookie, ClientsideTransfer, ClientsideUpdateTags, MultisideKeepAlive, ServersideAcknowledgeFinishConfiguration, ServersideClientInformation,
  ServersideCookieResponse, ServersideKnownPacks, ServersidePluginMessage, ServersidePong, ServersideResourcePackResponse,
};
use crate::{IntoPacket, Packet};

#[derive(Clone, Debug, PartialEq)]
pub enum ClientsideConfigurationPacket {
  CookieRequest(ClientsideCookieRequest),
  PluginMessage(ClientsidePluginMessage),
  Disconnect(ClientsideDisconnect),
  FinishConfiguration(ClientsideFinishConfiguration),
  KeepAlive(MultisideKeepAlive),
  Ping(ClientsidePing),
  ResetChat(ClientsideResetChat),
  RegistryData(ClientsideRegistryData),
  RemoveResourcePack(ClientsideRemoveResourcePack),
  AddResourcePack(ClientsideAddResourcePack),
  StoreCookie(ClientsideStoreCookie),
  Transfer(ClientsideTransfer),
  FeatureFlags(ClientsideFeatureFlags),
  UpdateTags(ClientsideUpdateTags),
  KnownPacks(ClientsideKnownPacks),
  CustomReportDetails(ClientsideCustomReportDetails),
  ServerLinks(ClientsideServerLinks),
}

impl Packet for ClientsideConfigurationPacket {
  fn id(&self) -> u32 {
    match self {
      Self::CookieRequest(_) => 0x00,
      Self::PluginMessage(_) => 0x01,
      Self::Disconnect(_) => 0x02,
      Self::FinishConfiguration(_) => 0x03,
      Self::KeepAlive(_) => 0x04,
      Self::Ping(_) => 0x05,
      Self::ResetChat(_) => 0x06,
      Self::RegistryData(_) => 0x07,
      Self::RemoveResourcePack(_) => 0x08,
      Self::AddResourcePack(_) => 0x09,
      Self::StoreCookie(_) => 0x0A,
      Self::Transfer(_) => 0x0B,
      Self::FeatureFlags(_) => 0x0C,
      Self::UpdateTags(_) => 0x0D,
      Self::KnownPacks(_) => 0x0E,
      Self::CustomReportDetails(_) => 0x0F,
      Self::ServerLinks(_) => 0x10,
    }
  }

  fn read(id: u32, buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x00 => Some(Self::CookieRequest(ClientsideCookieRequest::read(buffer)?)),
      0x01 => Some(Self::PluginMessage(ClientsidePluginMessage::read(buffer)?)),
      0x02 => Some(Self::Disconnect(ClientsideDisconnect::read(buffer)?)),
      0x03 => Some(Self::FinishConfiguration(ClientsideFinishConfiguration::read(buffer)?)),
      0x04 => Some(Self::KeepAlive(MultisideKeepAlive::read(buffer)?)),
      0x05 => Some(Self::Ping(ClientsidePing::read(buffer)?)),
      0x06 => Some(Self::ResetChat(ClientsideResetChat::read(buffer)?)),
      0x07 => Some(Self::RegistryData(ClientsideRegistryData::read(buffer)?)),
      0x08 => Some(Self::RemoveResourcePack(ClientsideRemoveResourcePack::read(buffer)?)),
      0x09 => Some(Self::AddResourcePack(ClientsideAddResourcePack::read(buffer)?)),
      0x0A => Some(Self::StoreCookie(ClientsideStoreCookie::read(buffer)?)),
      0x0B => Some(Self::Transfer(ClientsideTransfer::read(buffer)?)),
      0x0C => Some(Self::FeatureFlags(ClientsideFeatureFlags::read(buffer)?)),
      0x0D => Some(Self::UpdateTags(ClientsideUpdateTags::read(buffer)?)),
      0x0E => Some(Self::KnownPacks(ClientsideKnownPacks::read(buffer)?)),
      0x0F => Some(Self::CustomReportDetails(ClientsideCustomReportDetails::read(buffer)?)),
      0x10 => Some(Self::ServerLinks(ClientsideServerLinks::read(buffer)?)),
      _ => None,
    }
  }

  fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    match self {
      Self::CookieRequest(p) => p.write(buffer),
      Self::PluginMessage(p) => p.write(buffer),
      Self::Disconnect(p) => p.write(buffer),
      Self::FinishConfiguration(p) => p.write(buffer),
      Self::KeepAlive(p) => p.write(buffer),
      Self::Ping(p) => p.write(buffer),
      Self::ResetChat(p) => p.write(buffer),
      Self::RegistryData(p) => p.write(buffer),
      Self::RemoveResourcePack(p) => p.write(buffer),
      Self::AddResourcePack(p) => p.write(buffer),
      Self::StoreCookie(p) => p.write(buffer),
      Self::Transfer(p) => p.write(buffer),
      Self::FeatureFlags(p) => p.write(buffer),
      Self::UpdateTags(p) => p.write(buffer),
      Self::KnownPacks(p) => p.write(buffer),
      Self::CustomReportDetails(p) => p.write(buffer),
      Self::ServerLinks(p) => p.write(buffer),
    }
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideCookieRequest {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::CookieRequest(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsidePluginMessage {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::PluginMessage(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideDisconnect {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::Disconnect(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideFinishConfiguration {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::FinishConfiguration(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for MultisideKeepAlive {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::KeepAlive(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsidePing {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::Ping(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideResetChat {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::ResetChat(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideRegistryData {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::RegistryData(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideRemoveResourcePack {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::RemoveResourcePack(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideAddResourcePack {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::AddResourcePack(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideStoreCookie {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::StoreCookie(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideTransfer {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::Transfer(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideFeatureFlags {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::FeatureFlags(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideUpdateTags {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::UpdateTags(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideKnownPacks {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::KnownPacks(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideCustomReportDetails {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::CustomReportDetails(self)
  }
}

impl IntoPacket<ClientsideConfigurationPacket> for ClientsideServerLinks {
  fn sample(self) -> ClientsideConfigurationPacket {
    ClientsideConfigurationPacket::ServerLinks(self)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServersideConfigurationPacket {
  ClientInformation(ServersideClientInformation),
  CookieResponse(ServersideCookieResponse),
  PluginMessage(ServersidePluginMessage),
  AcknowledgeFinishConfiguration(ServersideAcknowledgeFinishConfiguration),
  KeepAlive(MultisideKeepAlive),
  Pong(ServersidePong),
  ResourcePackResponse(ServersideResourcePackResponse),
  KnownPacks(ServersideKnownPacks),
}

impl Packet for ServersideConfigurationPacket {
  fn id(&self) -> u32 {
    match self {
      Self::ClientInformation(_) => 0x00,
      Self::CookieResponse(_) => 0x01,
      Self::PluginMessage(_) => 0x02,
      Self::AcknowledgeFinishConfiguration(_) => 0x03,
      Self::KeepAlive(_) => 0x04,
      Self::Pong(_) => 0x05,
      Self::ResourcePackResponse(_) => 0x06,
      Self::KnownPacks(_) => 0x07,
    }
  }

  fn read(id: u32, buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    match id {
      0x00 => Some(Self::ClientInformation(ServersideClientInformation::read(buffer)?)),
      0x01 => Some(Self::CookieResponse(ServersideCookieResponse::read(buffer)?)),
      0x02 => Some(Self::PluginMessage(ServersidePluginMessage::read(buffer)?)),
      0x03 => Some(Self::AcknowledgeFinishConfiguration(ServersideAcknowledgeFinishConfiguration::read(buffer)?)),
      0x04 => Some(Self::KeepAlive(MultisideKeepAlive::read(buffer)?)),
      0x05 => Some(Self::Pong(ServersidePong::read(buffer)?)),
      0x06 => Some(Self::ResourcePackResponse(ServersideResourcePackResponse::read(buffer)?)),
      0x07 => Some(Self::KnownPacks(ServersideKnownPacks::read(buffer)?)),
      _ => None,
    }
  }

  fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    match self {
      Self::ClientInformation(p) => p.write(buffer),
      Self::CookieResponse(p) => p.write(buffer),
      Self::PluginMessage(p) => p.write(buffer),
      Self::AcknowledgeFinishConfiguration(p) => p.write(buffer),
      Self::KeepAlive(p) => p.write(buffer),
      Self::Pong(p) => p.write(buffer),
      Self::ResourcePackResponse(p) => p.write(buffer),
      Self::KnownPacks(p) => p.write(buffer),
    }
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersideClientInformation {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::ClientInformation(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersideCookieResponse {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::CookieResponse(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersidePluginMessage {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::PluginMessage(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersideAcknowledgeFinishConfiguration {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::AcknowledgeFinishConfiguration(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for MultisideKeepAlive {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::KeepAlive(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersidePong {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::Pong(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersideResourcePackResponse {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::ResourcePackResponse(self)
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersideKnownPacks {
  fn sample(self) -> ServersideConfigurationPacket {
    ServersideConfigurationPacket::KnownPacks(self)
  }
}
impl IntoPacket<ClientsideConfigurationPacket> for ClientsideConfigurationPacket {
  fn sample(self) -> ClientsideConfigurationPacket {
    self
  }
}

impl IntoPacket<ServersideConfigurationPacket> for ServersideConfigurationPacket {
  fn sample(self) -> ServersideConfigurationPacket {
    self
  }
}
