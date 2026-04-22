use std::io::{self, Cursor, Write};

use nurtex_codec::{Buffer, VarInt};

/// Действие игрока
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PlayerAction {
  StartedDigging,
  CancelledDigging,
  FinishedDigging,
  DropItemStack,
  DropItem,
  FinishUsingItem,
  SwapItem,
}

impl Buffer for PlayerAction {
  fn read_buf(buffer: &mut Cursor<&[u8]>) -> Option<Self> {
    let id = i32::read_varint(buffer)?;

    Some(match id {
      0 => Self::StartedDigging,
      1 => Self::CancelledDigging,
      2 => Self::FinishedDigging,
      3 => Self::DropItemStack,
      4 => Self::DropItem,
      5 => Self::FinishUsingItem,
      6 => Self::SwapItem,
      _ => return None,
    })
  }

  fn write_buf(&self, buffer: &mut impl Write) -> io::Result<()> {
    let id = match self {
      Self::StartedDigging => 0,
      Self::CancelledDigging => 1,
      Self::FinishedDigging => 2,
      Self::DropItemStack => 3,
      Self::DropItem => 4,
      Self::FinishUsingItem => 5,
      Self::SwapItem => 6,
    };

    id.write_varint(buffer)?;

    Ok(())
  }
}
