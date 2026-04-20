use nurtex_derive::PacketUnion;

use crate::packets::play::{
  ClientsideChunkCacheCenter, ClientsideChunkCacheRadius, ClientsideDamageEvent, ClientsideDisconnect, 
  ClientsideLogin, ClientsideMoveVehicle, ClientsideOpenContainer, ClientsidePing, ClientsidePingResponse, 
  ClientsidePlayerChat, ClientsidePlayerCombatKill, ClientsidePlayerLookAt, ClientsidePlayerPosition, ClientsidePlayerRotation, 
  ClientsideRemoveEntities, ClientsideRemoveEntityEffect, ClientsideRotateHead, ClientsideSectionBlocksUpdate, 
  ClientsideSetCamera, ClientsideAddResourcePack, ClientsideRemoveResourcePack, ClientsideSetEntityLink, ClientsideSetEntityVelocity, 
  ClientsideSetExperience, ClientsideSetHealth, ClientsideSetPassengers, ClientsideSpawnEntity, ClientsideTransfer, 
  ClientsideUpdateEntityPos, ClientsideUpdateEntityPosRot, ClientsideUpdateEntityRot, MultisideKeepAlive, 
  ServersideAcceptTeleportation, ServersideClientCommand, ServersideMovePlayerPos, ServersideMovePlayerPosRot, ServersideMovePlayerRot,
  ServersideMovePlayerStatusOnly, ServersidePingRequest, ServersidePong, ServersideSwingArm, ServersideUseItem
};

#[derive(Clone, Debug, PartialEq, PacketUnion)]
pub enum ClientsidePlayPacket {
  #[packet_id = 0x2B]
  KeepAlive(MultisideKeepAlive),
  #[packet_id = 0x3B]
  Ping(ClientsidePing),
  #[packet_id = 0x3C]
  PingResponse(ClientsidePingResponse),
  #[packet_id = 0x19]
  DamageEvent(ClientsideDamageEvent),
  #[packet_id = 0x33]
  UpdateEntityPos(ClientsideUpdateEntityPos),
  #[packet_id = 0x36]
  UpdateEntityRot(ClientsideUpdateEntityRot),
  #[packet_id = 0x34]
  UpdateEntityPosRot(ClientsideUpdateEntityPosRot),
  #[packet_id = 0x46]
  PlayerPosition(ClientsidePlayerPosition),
  #[packet_id = 0x47]
  PlayerRotation(ClientsidePlayerRotation),
  #[packet_id = 0x45]
  PlayerLookAt(ClientsidePlayerLookAt),
  #[packet_id = 0x42]
  PlayerCombatKill(ClientsidePlayerCombatKill),
  #[packet_id = 0x66]
  SetHealth(ClientsideSetHealth),
  #[packet_id = 0x65]
  SetExperience(ClientsideSetExperience),
  #[packet_id = 0x69]
  SetPassengers(ClientsideSetPassengers),
  #[packet_id = 0x63]
  SetEntityVelocity(ClientsideSetEntityVelocity),
  #[packet_id = 0x01]
  SpawnEntity(ClientsideSpawnEntity),
  #[packet_id = 0x4B]
  RemoveEntities(ClientsideRemoveEntities),
  #[packet_id = 0x20]
  Disconnect(ClientsideDisconnect),
  #[packet_id = 0x3F]
  PlayerChat(ClientsidePlayerChat),
  #[packet_id = 0x7F]
  Transfer(ClientsideTransfer),
  #[packet_id = 0x62]
  SetEntityLink(ClientsideSetEntityLink),
  #[packet_id = 0x5D]
  ChunkCacheRadius(ClientsideChunkCacheRadius),
  #[packet_id = 0x5C]
  ChunkCacheCenter(ClientsideChunkCacheCenter),
  #[packet_id = 0x5B]
  SetCamera(ClientsideSetCamera),
  #[packet_id = 0x51]
  RotateHead(ClientsideRotateHead),
  #[packet_id = 0x52]
  SectionBlocksUpdate(ClientsideSectionBlocksUpdate),
  #[packet_id = 0x4F]
  AddResourcePack(ClientsideAddResourcePack),
  #[packet_id = 0x4E]
  RemoveResourcePack(ClientsideRemoveResourcePack),
  #[packet_id = 0x4C]
  RemoveEntityEffect(ClientsideRemoveEntityEffect),
  #[packet_id = 0x39]
  OpenContainer(ClientsideOpenContainer),
  #[packet_id = 0x37]
  MoveVehicle(ClientsideMoveVehicle),
  #[packet_id = 0x30]
  Login(ClientsideLogin),
}

#[derive(Clone, Debug, PartialEq, PacketUnion)]
pub enum ServersidePlayPacket {
  #[packet_id = 0x1B]
  KeepAlive(MultisideKeepAlive),
  #[packet_id = 0x2C]
  Pong(ServersidePong),
  #[packet_id = 0x25]
  PingRequest(ServersidePingRequest),
  #[packet_id = 0x00]
  AcceptTeleportation(ServersideAcceptTeleportation),
  #[packet_id = 0x3C]
  SwingArm(ServersideSwingArm),
  #[packet_id = 0x40]
  UseItem(ServersideUseItem),
  #[packet_id = 0x1D]
  MovePlayerPos(ServersideMovePlayerPos),
  #[packet_id = 0x1F]
  MovePlayerRot(ServersideMovePlayerRot),
  #[packet_id = 0x1E]
  MovePlayerPosRot(ServersideMovePlayerPosRot),
  #[packet_id = 0x20]
  MovePlayerStatusOnly(ServersideMovePlayerStatusOnly),
  #[packet_id = 0x0B]
  ClientCommand(ServersideClientCommand),
}
