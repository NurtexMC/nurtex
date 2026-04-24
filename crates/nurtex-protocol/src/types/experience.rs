use nurtex_codec::VarInt;

use std::io::{self, Cursor, Write};

use nurtex_codec::Buffer;

/// Структура опыта
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Experience {
  pub bar: f32,
  pub level: i32,
  pub total: i32,
}

impl Default for Experience {
  fn default() -> Self {
    Self { bar: 0.0, level: 0, total: 0 }
  }
}

impl Buffer for Experience {
  fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      bar: f32::read_buf(buffer)?,
      level: i32::read_varint(buffer)?,
      total: i32::read_varint(buffer)?,
    })
  }

  fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.bar.write_buf(buffer)?;
    self.level.write_varint(buffer)?;
    self.total.write_varint(buffer)?;
    Ok(())
  }
}
