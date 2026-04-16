use std::io::{self, Cursor, Write};

use nurtex_codec::{Buffer, BufferVar};

#[derive(Clone, Debug, PartialEq)]
pub struct MultisideKeepAlive {
  pub id: i64,
}

impl MultisideKeepAlive {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self { id: i64::read_buf(buffer)? })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.id.write_buf(buffer)?;
    Ok(())
  }
}

// Знаю что можно объединить `ClientsidePing` с `ServersidePong`
// и `ClientsidePingResponse` с `ServersidePingRequest`, просто так
// будет трудно различать их :)

#[derive(Clone, Debug, PartialEq)]
pub struct ClientsidePing {
  pub id: i32,
}

impl ClientsidePing {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self { id: i32::read_buf(buffer)? })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.id.write_buf(buffer)?;
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientsidePingResponse {
  pub timestamp: i64,
}

impl ClientsidePingResponse {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      timestamp: i64::read_buf(buffer)?,
    })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.timestamp.write_buf(buffer)?;
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientsideSyncPlayerPosition {
  pub teleport_id: i64,
  pub position_x: f64,
  pub position_y: f64,
  pub position_z: f64,
  pub velocity_x: f64,
  pub velocity_y: f64,
  pub velocity_z: f64,
  pub yaw: f32,
  pub pitch: f32,
  pub teleport_flags: TeleportFlags,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TeleportFlags {
  pub relative_x: bool,
  pub relative_y: bool,
  pub relative_z: bool,
  pub relative_yaw: bool,
  pub relative_pitch: bool,
  pub relative_velocity_x: bool,
  pub relative_velocity_y: bool,
  pub relative_velocity_z: bool,
  pub rotate_velocity: bool,
}

impl TeleportFlags {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let flags = i32::read_buf(buffer)?;
    Some(Self {
      relative_x: (flags & 0x0001) != 0,
      relative_y: (flags & 0x0002) != 0,
      relative_z: (flags & 0x0004) != 0,
      relative_yaw: (flags & 0x0008) != 0,
      relative_pitch: (flags & 0x0010) != 0,
      relative_velocity_x: (flags & 0x0020) != 0,
      relative_velocity_y: (flags & 0x0040) != 0,
      relative_velocity_z: (flags & 0x0080) != 0,
      rotate_velocity: (flags & 0x0100) != 0,
    })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    let mut flags = 0i32;
    if self.relative_x {
      flags |= 0x0001;
    }
    if self.relative_y {
      flags |= 0x0002;
    }
    if self.relative_z {
      flags |= 0x0004;
    }
    if self.relative_yaw {
      flags |= 0x0008;
    }
    if self.relative_pitch {
      flags |= 0x0010;
    }
    if self.relative_velocity_x {
      flags |= 0x0020;
    }
    if self.relative_velocity_y {
      flags |= 0x0040;
    }
    if self.relative_velocity_z {
      flags |= 0x0080;
    }
    if self.rotate_velocity {
      flags |= 0x0100;
    }
    flags.write_buf(buffer)
  }
}

impl ClientsideSyncPlayerPosition {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      teleport_id: i64::read_varint(buffer)?,
      position_x: f64::read_buf(buffer)?,
      position_y: f64::read_buf(buffer)?,
      position_z: f64::read_buf(buffer)?,
      velocity_x: f64::read_buf(buffer)?,
      velocity_y: f64::read_buf(buffer)?,
      velocity_z: f64::read_buf(buffer)?,
      yaw: f32::read_buf(buffer)?,
      pitch: f32::read_buf(buffer)?,
      teleport_flags: TeleportFlags::read(buffer)?,
    })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.teleport_id.write_varint(buffer)?;
    self.position_x.write_buf(buffer)?;
    self.position_y.write_buf(buffer)?;
    self.position_z.write_buf(buffer)?;
    self.velocity_x.write_buf(buffer)?;
    self.velocity_y.write_buf(buffer)?;
    self.velocity_z.write_buf(buffer)?;
    self.yaw.write_buf(buffer)?;
    self.pitch.write_buf(buffer)?;
    self.teleport_flags.write(buffer)?;
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServersidePong {
  pub id: i32,
}

impl ServersidePong {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self { id: i32::read_buf(buffer)? })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.id.write_buf(buffer)?;
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServersidePingRequest {
  pub timestamp: i64,
}

impl ServersidePingRequest {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      timestamp: i64::read_buf(buffer)?,
    })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.timestamp.write_buf(buffer)?;
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServersideAcceptTeleportation {
  pub teleport_id: i64,
}

impl ServersideAcceptTeleportation {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      teleport_id: i64::read_varint(buffer)?,
    })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.teleport_id.write_varint(buffer)?;
    Ok(())
  }
}
