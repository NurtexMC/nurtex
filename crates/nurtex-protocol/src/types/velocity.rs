use std::io::{self, Cursor, Write};

use nurtex_codec::Buffer;

/// Структура скорости
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Velocity {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Velocity {
  /// Метод создания нового экземпляра `Velocity`
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }

  /// Метод создания нулевой скорости
  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0 }
  }

  /// Метод вычисления разницы между текущей и другой скорости
  pub fn delta(&self, other: Velocity) -> Self {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    let dz = self.z - other.z;

    Self { x: dx, y: dy, z: dz }
  }

  /// Метод чтения `Velocity` из буффера
  pub fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    Some(Self {
      x: f64::read_buf(buffer)?,
      y: f64::read_buf(buffer)?,
      z: f64::read_buf(buffer)?,
    })
  }

  /// Метод записи `Velocity` в буффер
  pub fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    self.x.write_buf(buffer)?;
    self.y.write_buf(buffer)?;
    self.z.write_buf(buffer)?;
    Ok(())
  }
}
