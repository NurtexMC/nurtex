use nurtex_codec::Buffer;
use nurtex_derive::Packet;
use uuid::Uuid;

use crate::types::{AdditionalMessageInfo, BlockPosition, ClientCommand, Experience, Face, GameEvent, InteractType, Item, LpVector3, PhysicsFlags, PlayerAction, PlayerCommand, RelativeHand, ResourcePackState, Rotation, TeleportFlags, TextComponent, Vector3};

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct MultisideKeepAlive {
  pub id: i64,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePing {
  pub id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePingResponse {
  pub timestamp: i64,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideDamageEvent {
  #[packet(varint)]
  pub entity_id: i32,
  #[packet(varint)]
  pub source_type_id: i32,
  #[packet(varint)]
  pub source_cause_id: i32,
  #[packet(varint)]
  pub source_direct_id: i32,
  pub source_position: Vector3,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideUpdateEntityPos {
  #[packet(varint)]
  pub entity_id: i32,
  pub delta_x: i16,
  pub delta_y: i16,
  pub delta_z: i16,
  pub on_ground: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideUpdateEntityRot {
  #[packet(varint)]
  pub entity_id: i32,
  pub yaw_angle: i8,
  pub pitch_angle: i8,
  pub on_ground: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideUpdateEntityPosRot {
  #[packet(varint)]
  pub entity_id: i32,
  pub delta_x: i16,
  pub delta_y: i16,
  pub delta_z: i16,
  pub yaw_angle: i8,
  pub pitch_angle: i8,
  pub on_ground: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePlayerPosition {
  #[packet(varlong)]
  pub teleport_id: i64,
  pub position: Vector3,
  pub velocity: Vector3,
  pub rotation: Rotation,
  pub teleport_flags: TeleportFlags,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePlayerRotation {
  pub yaw: f32,
  pub relative_yaw: bool,
  pub pitch: f32,
  pub relative_pitch: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePlayerLookAt {
  #[packet(varint)]
  pub gaze: i32,
  pub target_pos: Vector3,
  pub is_entity: bool,
  #[packet(varint)]
  pub entity_id: Option<i32>,
  #[packet(varint)]
  pub entity_gaze: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePlayerCombatKill {
  #[packet(varint)]
  pub player_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetHealth {
  pub health: f32,
  #[packet(varint)]
  pub food: i32,
  pub food_saturation: f32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetExperience {
  pub experience: Experience,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetPassengers {
  #[packet(varint)]
  pub entity_id: i32,
  #[packet(vec_varint)]
  pub passengers: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetEntityVelocity {
  #[packet(varint)]
  pub entity_id: i32,
  pub velocity: LpVector3,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSpawnEntity {
  #[packet(varint)]
  pub entity_id: i32,
  pub entity_uuid: Uuid,
  #[packet(varint)]
  pub entity_type: i32,
  pub position: Vector3,
  pub velocity: LpVector3,
  pub angle_pitch: i8,
  pub angle_yaw: i8,
  pub angle_head_yaw: i8,
  #[packet(varint)]
  pub data: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideRemoveEntities {
  #[packet(vec_varint)]
  pub entities: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideDisconnect {
  // Я не знаю почему, но TextComponent не подходит здесь под поле `reason`, 
  // хотя в тех же пакетах в состоянии Configuration / Login это работает
  // -----------------------------------------------------------------------
  // pub reason: TextComponent,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsidePlayerChat {
  #[packet(varint)]
  pub global_index: i32,
  pub sender_uuid: Uuid,
  #[packet(varint)]
  pub index: i32,
  pub message_signature: Option<Vec<u8>>,
  pub message: String,
  pub timestamp: i64,
  pub salt: i64,
  #[packet(varint)]
  pub message_id: i32,
  pub signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSystemChat {
  pub message: TextComponent,
  pub overlay: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideTransfer {
  pub server_host: String,
  #[packet(varint)]
  pub server_port: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetHeldItem {
  #[packet(varint)]
  pub slot: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetEntityLink {
  #[packet(varint)]
  pub attached_entity_id: i32,
  #[packet(varint)]
  pub holding_entity_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideChunkCacheRadius {
  #[packet(varint)]
  pub view_distance: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideChunkCacheCenter {
  #[packet(varint)]
  pub chunk_x: i32,
  #[packet(varint)]
  pub chunk_z: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSetCamera {
  #[packet(varint)]
  pub camera_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideRotateHead {
  #[packet(varint)]
  pub entity_id: i32,
  pub head_yaw: i8,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideSectionBlocksUpdate {
  pub chunk_section_position: i64,
  #[packet(vec_varlong)]
  pub head_yaw: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideAddResourcePack {
  pub uuid: uuid::Uuid,
  pub url: String,
  pub hash: String,
  pub forced: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideRemoveResourcePack {
  pub uuid: uuid::Uuid,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideRemoveEntityEffect {
  #[packet(varint)]
  pub entity_id: i32,
  #[packet(varint)]
  pub effect_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideOpenContainer {
  #[packet(varint)]
  pub window_id: i32,
  #[packet(varint)]
  pub window_type: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideMoveVehicle {
  pub position: Vector3,
  pub rotation: Rotation,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideLogin {
  pub entity_id: i32,
  pub is_hardcore: bool,
  pub dimension_names: Vec<String>,
  #[packet(varint)]
  pub max_players: i32,
  #[packet(varint)]
  pub view_distance: i32,
  #[packet(varint)]
  pub simulation_distance: i32,
  pub reduced_debug_info: bool,
  pub enable_respawn_screen: bool,
  #[packet(varint)]
  pub dimension_type: i32,
  pub dimension_name: String,
  pub hashed_seed: i64,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideEntityPositionSync {
  #[packet(varint)]
  pub entity_id: i32,
  pub position: Vector3,
  pub velocity: Vector3,
  pub rotation: Rotation,
  pub on_ground: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideExplosion {
  pub position: Vector3,
  pub radius: f32,
  pub block_count: i32,
  pub player_delta_velocity: Option<Vector3>,
  #[packet(varint)]
  pub explosion_particle_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideUnloadChunk {
  pub chunk_x: i32,
  pub chunk_z: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideGameEvent {
  pub event: GameEvent,
  pub value: f32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideClearChat {
  #[packet(varint)]
  pub message_id: i32,
  pub signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideLoadChunkWithLight {
  pub chunk_x: i32,
  pub chunk_z: i32,
  pub chunk_data: Vec<u8>,
  pub light_data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideBlockUpdate {
  pub block_pos: BlockPosition,
  #[packet(varint)]
  pub block_state: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideContainerSetContent {
  #[packet(varint)]
  pub container_id: i32,
  #[packet(varint)]
  pub state_id: i32,
  pub items: Vec<Item>,
  pub carried_item: Item,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideContainerSetSlot {
  #[packet(varint)]
  pub container_id: i32,
  #[packet(varint)]
  pub state_id: i32,
  pub slot: i16,
  pub item: Item,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideChunkBatchStart;

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideChunkBatchFinished {
  #[packet(varint)]
  pub batch_size: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideLightUpdate {
  #[packet(varint)]
  pub chunk_x: i32,
  #[packet(varint)]
  pub chunk_z: i32,
  pub light_data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideCloseContainer {
  #[packet(varint)]
  pub container_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ClientsideContainerSetData {
  #[packet(varint)]
  pub container_id: i32,
  #[packet(varint)]
  pub property: i32,
  #[packet(varint)]
  pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersidePong {
  pub id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersidePingRequest {
  pub timestamp: i64,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideAcceptTeleportation {
  #[packet(varlong)]
  pub teleport_id: i64,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideSwingArm {
  pub hand: RelativeHand,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideUseItem {
  pub hand: RelativeHand,
  #[packet(varint)]
  pub sequence: i32,
  pub rotation: Rotation,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideMovePlayerPos {
  pub position: Vector3,
  pub flags: PhysicsFlags,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideMovePlayerRot {
  pub rotation: Rotation,
  pub flags: PhysicsFlags,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideMovePlayerPosRot {
  pub position: Vector3,
  pub rotation: Rotation,
  pub flags: PhysicsFlags,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideMovePlayerStatusOnly {
  pub flags: PhysicsFlags,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideClientCommand {
  pub command: ClientCommand,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideChatCommand {
  pub command: String,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideChatMessage {
  pub message: String,
  pub timestamp: i64,
  pub salt: i64,
  pub signature: Option<Vec<u8>>,
  pub additional_info: AdditionalMessageInfo,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideSetHeldItem {
  pub slot: i16,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideInteract {
  #[packet(varint)]
  pub entity: i32,
  pub interact_type: InteractType,
  pub target_x: Option<f32>,
  pub target_y: Option<f32>,
  pub target_z: Option<f32>,
  pub hand: Option<RelativeHand>,
  pub sneak_key_pressed: bool,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersidePlayerAction {
  pub action: PlayerAction,
  pub block_pos: BlockPosition,
  pub face: Face,
  #[packet(varint)]
  pub sequence: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersidePlayerCommand {
  #[packet(varint)]
  pub entity_id: i32,
  pub command: PlayerCommand,
  #[packet(varint)]
  pub jump_boost: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideResourcePackResponse {
  pub uuid: Uuid,
  pub state: ResourcePackState,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideContainerClick {
  #[packet(varint)]
  pub container_id: i32,
  #[packet(varint)]
  pub state_id: i32,
  pub slot: i16,
  pub button: i8,
  #[packet(varint)]
  pub mode: i32,
  pub clicked_item: Item,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideContainerClose {
  #[packet(varint)]
  pub container_id: i32,
}

#[derive(Clone, Debug, PartialEq, Packet)]
pub struct ServersideEditBook {
  pub slot: i32,
  pub pages: Vec<String>,
  pub title: Option<String>,
}
