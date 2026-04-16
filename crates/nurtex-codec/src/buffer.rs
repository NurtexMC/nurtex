use std::io::{self, Cursor, Error, ErrorKind, Read, Write};

/// Трейт буффера
pub trait Buffer
where
  Self: Sized,
{
  fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self>;
  fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()>;
}

/// Трейт var буффера
pub trait BufferVar
where
  Self: Sized,
{
  fn read_varint(buffer: &mut Cursor<&[u8]>) -> Option<Self>;
  fn write_varint(&self, buffer: &mut impl Write) -> io::Result<()>;
}

/// Вспомогательная функция для чтения одного байта из буффера
pub fn read_byte(buffer: &mut Cursor<&[u8]>) -> Option<u8> {
  let mut buf = [0u8; 1];
  buffer.read_exact(&mut buf).ok()?;
  Some(buf[0])
}

/// Вспомогательная функция чтения байтов из буффера
pub fn read_bytes<'a>(buffer: &'a mut Cursor<&[u8]>, length: usize) -> Option<&'a [u8]> {
  if length > (buffer.get_ref().len() - buffer.position() as usize) {
    return None;
  }

  let initial_position = buffer.position() as usize;
  buffer.set_position(buffer.position() + length as u64);
  let data = &buffer.get_ref()[initial_position..initial_position + length];

  Some(data)
}

/// Вспомогательная функция чтения строки из буффера
pub fn read_str<'a>(buffer: &'a mut Cursor<&[u8]>) -> Option<&'a str> {
  let length = u32::read_varint(buffer)?;

  if length > 32767 * 4 {
    return None;
  }

  let buffer = read_bytes(buffer, length as usize)?;
  let string = std::str::from_utf8(buffer).ok()?;

  if string.len() > length as usize {
    return None;
  }

  Some(string)
}

/// Вспомогательная функция записи строки в буффер
pub fn write_str(buffer: &mut impl Write, string: &str) -> io::Result<()> {
  let str_len = string.len();

  if str_len > 32767 {
    return Err(Error::new(ErrorKind::InvalidData, ""));
  }

  string.as_bytes().to_vec().write_buf(buffer)?;

  Ok(())
}
