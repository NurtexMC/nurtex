use std::io::{self, Cursor, Write};

use nurtex_codec::{Buffer, VarInt};

/// Предмет инвентаря
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
  Null,
  Some { count: i32, id: i32 },
}

impl Buffer for Item {
  fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    if !bool::read_buf(buffer)? {
      return Some(Self::Null);
    }

    Some(Self::Some {
      count: i32::read_varint(buffer)?,
      id: i32::read_varint(buffer)?,
    })
  }

  fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    match self {
      Self::Null => {
        false.write_buf(buffer)?;
      }
      Self::Some { count, id } => {
        true.write_buf(buffer)?;
        count.write_varint(buffer)?;
        id.write_varint(buffer)?;
      }
    }

    Ok(())
  }
}
