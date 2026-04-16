use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};

const DEFAULT_SOCKET_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565));

pub trait NurtexAddress {
  fn to_socket_addr(&self) -> Option<SocketAddr>;
  fn force_to_socket_addr(&self) -> SocketAddr;
}

impl NurtexAddress for String {
  fn to_socket_addr(&self) -> Option<SocketAddr> {
    let mut iter = self.to_socket_addrs().ok()?;
    iter.next()
  }

  fn force_to_socket_addr(&self) -> SocketAddr {
    if let Ok(mut iter) = self.to_socket_addrs() {
      iter.next().unwrap_or(DEFAULT_SOCKET_ADDR)
    } else {
      DEFAULT_SOCKET_ADDR
    }
  }
}

/// Вспомогательная функция конвертации `&str` или `String` адреса в `SocketAddr`
pub fn convert_address(address: impl Into<String>) -> Option<SocketAddr> {
  address.into().to_socket_addr()
}
