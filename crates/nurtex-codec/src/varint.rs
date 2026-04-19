use crate::{CONTINUE_BIT, SEGMENT_BITS, read_byte};
use std::io::{self, Cursor, Write};

/// Обёртка для типа `VarInt`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt(i32);

impl VarInt {
  /// Метод создания нового экземпляра `VarInt`
  pub fn new(value: i32) -> Self {
    Self(value)
  }

  /// Метод создания экземпляра `VarInt` из определённого числа
  pub fn from(value: impl Into<i32>) -> Self {
    Self(value.into())
  }

  /// Метод получения значения
  pub fn value(&self) -> i32 {
    self.0
  }

  /// Метод чтения `VarInt` из буффера
  pub fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let mut value = 0i32;
    let mut position = 0u32;

    loop {
      let byte = read_byte(buffer)?;
      value |= (((byte & SEGMENT_BITS) as u32) << position) as i32;

      if (byte & CONTINUE_BIT) == 0 {
        break;
      }

      position += 7;

      if position >= 32 {
        return None;
      }
    }

    Some(Self(value))
  }

  /// Метод записи `VarInt` в буффер
  pub fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    let mut array = [0];
    let mut value = self.0;

    if value == 0 {
      buffer.write_all(&array)?;
      return Ok(());
    }

    while value != 0 {
      array[0] = (value & SEGMENT_BITS as i32) as u8;
      value = (value >> 7) & (i32::MAX >> 6);

      if value != 0 {
        array[0] |= CONTINUE_BIT;
      }

      buffer.write_all(&array)?;
    }

    Ok(())
  }
}
