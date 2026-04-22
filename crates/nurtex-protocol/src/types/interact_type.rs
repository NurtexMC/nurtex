use std::io::{self, Cursor, Write};

use nurtex_codec::{Buffer, VarInt};

/// Тип взаимодействия
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InteractType {
  Interact,
  Attack,
  InteractAt,
}

impl Buffer for InteractType {
  fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let id = i32::read_varint(buffer)?;

    Some(match id {
      0 => Self::Interact,
      1 => Self::Attack,
      2 => Self::InteractAt,
      _ => return None,
    })
  }

  fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    let id = match self {
      Self::Interact => 0,
      Self::Attack => 1,
      Self::InteractAt => 2,
    };

    id.write_varint(buffer)?;

    Ok(())
  }
}
