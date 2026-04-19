use crate::{CONTINUE_BIT, SEGMENT_BITS, read_byte};
use std::io::{self, Cursor, Write};

/// Обёртка для типа `VarLong`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarLong(i64);

impl VarLong {
  /// Метод создания нового экземпляра `VarLong`
  pub fn new(value: i64) -> Self {
    Self(value)
  }

  /// Метод создания экземпляра `VarLong` из определённого числа
  pub fn from(value: impl Into<i64>) -> Self {
    Self(value.into())
  }

  /// Метод получения значения
  pub fn value(&self) -> i64 {
    self.0
  }

  /// Метод чтения `VarLong` из буффера
  pub fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let mut value = 0i64;
    let mut position = 0u32;

    loop {
      let byte = read_byte(buffer)?;
      value |= (((byte & SEGMENT_BITS) as u32) << position) as i64;

      if (byte & CONTINUE_BIT) == 0 {
        break;
      }

      position += 7;

      if position >= 64 {
        return None;
      }
    }

    Some(Self(value))
  }

  /// Метод записи `VarLong` в буффер
  pub fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    let mut array = [0];
    let mut value = self.0;

    if value == 0 {
      buffer.write_all(&array)?;
      return Ok(());
    }

    while value != 0 {
      array[0] = (value & SEGMENT_BITS as i64) as u8;
      value = (value >> 7) & (i64::MAX >> 6);

      if value != 0 {
        array[0] |= CONTINUE_BIT;
      }

      buffer.write_all(&array)?;
    }

    Ok(())
  }
}
