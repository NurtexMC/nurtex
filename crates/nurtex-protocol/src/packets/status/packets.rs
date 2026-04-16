use nurtex_codec::Buffer;
use std::io::{self, Cursor, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct ClientsidePongResponse {
  pub timestamp: i64,
}

impl ClientsidePongResponse {
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
pub struct ClientsideStatusResponse {
  pub response: String,
}

impl ClientsideStatusResponse {
  pub fn read(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      response: String::read_buf(buffer)?,
    })
  }

  pub fn write(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.response.write_buf(buffer)?;
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServersideStatusRequest;

impl ServersideStatusRequest {
  pub fn read(_buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self)
  }

  pub fn write(&self, _buffer: &mut impl Write) -> io::Result<()> {
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
