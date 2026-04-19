use std::io::{self, Cursor, Write};

use nurtex_codec::VarInt;

/// Точная рука игрока (левая / правая)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AccurateHand {
  Left,
  Right,
}

impl AccurateHand {
  /// Метод чтения `AccurateHand` из буффера
  pub fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let id = VarInt::read_buf(buffer)?.value();

    match id {
      0 => Some(Self::Left),
      1 => Some(Self::Right),
      _ => None,
    }
  }

  /// Метод записи `AccurateHand` в буффер
  pub fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    let id = match self {
      Self::Left => 0,
      Self::Right => 1,
    };

    VarInt::new(id).write_buf(buffer)?;

    Ok(())
  }
}

/// Относительная рука игрока
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RelativeHand {
  MainHand,
  OffHand,
}

impl RelativeHand {
  /// Метод чтения `RelativeHand` из буффера
  pub fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let id = VarInt::read_buf(buffer)?.value();

    match id {
      0 => Some(Self::MainHand),
      1 => Some(Self::OffHand),
      _ => None,
    }
  }

  /// Метод записи `RelativeHand` в буффер
  pub fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    let id = match self {
      Self::MainHand => 0,
      Self::OffHand => 1,
    };

    VarInt::new(id).write_buf(buffer)?;

    Ok(())
  }
}
