use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};

/// Адрес сокета по умолчанию (локальный хост, `127.0.0.1:25565`)
const DEFAULT_SOCKET_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565));

/// Вспомогательная структура для удобной работы с `SocketAddr`
pub struct NurtexAddr(SocketAddr);

impl NurtexAddr {
  /// Метод получения `NurtexAddr` из строки
  pub fn from(input: impl Into<String>) -> Self {
    if let Some(mut iter) = input.into().to_socket_addrs().ok() {
      Self(iter.next().unwrap_or(DEFAULT_SOCKET_ADDR))
    } else {
      Self(DEFAULT_SOCKET_ADDR)
    }
  }

  /// Безопасный метод получения `Option<NurtexAddr>` из строки
  pub fn try_from(input: impl Into<String>) -> Option<Self> {
    if let Some(mut iter) = input.into().to_socket_addrs().ok() {
      if let Some(addr) = iter.next() {
        return Some(Self(addr));
      }
    }

    None
  }

  /// Метод получения `SocketAddr` 
  pub fn unpack(&self) -> SocketAddr {
    self.0
  }

  /// Метод получения IP-адреса (например `127.0.0.1`)
  pub fn get_ip(&self) -> String {
    self.0.ip().to_string()
  }

  /// Метод получения порта (например `25565`)
  pub fn get_port(&self) -> u16 {
    self.0.port()
  }
}

/// Вспомогательная функция конвертации строкового адреса в `NurtexAddr`
pub fn convert_address(address: impl Into<String>) -> Option<NurtexAddr> {
  NurtexAddr::try_from(address)
}
