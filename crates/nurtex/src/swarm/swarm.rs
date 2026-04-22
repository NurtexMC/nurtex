use std::io::{self, Error};
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;

use crate::bot::Bot;
use crate::swarm::{Speedometer, SwarmObject};

/// Структура роя ботов
pub struct Swarm {
  /// Список всех ботов
  pub bots: Vec<Arc<Bot>>,

  /// Список всех хэндлов
  handles: Vec<JoinHandle<Result<(), Error>>>,

  /// Спидометр (опционально)
  speedometer: Option<Arc<Speedometer>>,
}

impl Swarm {
  /// Метод создания нового роя
  pub fn create() -> Self {
    Self {
      bots: Vec::new(),
      handles: Vec::new(),
      speedometer: None,
    }
  }

  /// Метод создания нового роя со спидометром
  pub fn create_with_speedometer(speedometer: Arc<Speedometer>) -> Self {
    Self {
      bots: Vec::new(),
      handles: Vec::new(),
      speedometer: Some(speedometer),
    }
  }

  /// Метод установки спидометра
  pub fn set_speedometer(&mut self, speedometer: Arc<Speedometer>) {
    self.speedometer = Some(speedometer);
  }

  /// Метод получения спидометра
  pub fn get_speedometer(&self) -> Option<&Arc<Speedometer>> {
    self.speedometer.as_ref()
  }

  /// Последовательный for-each
  pub async fn for_each_consistent<F, Fut>(&self, f: F)
  where
    F: Fn(Arc<Bot>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
  {
    for i in &self.bots {
      let bot = Arc::clone(i);
      f(bot).await;
    }
  }

  /// Параллельный for-each
  pub fn for_each_parallel<F, Fut>(&self, f: F)
  where
    F: Fn(Arc<Bot>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
  {
    let f = Arc::new(f);

    for i in &self.bots {
      let f_clone = Arc::clone(&f);
      let bot = Arc::clone(i);

      tokio::spawn(f_clone(bot));
    }
  }

  /// Метод добавления объекта бота в рой
  pub fn add_object(&mut self, object: SwarmObject) {
    let bot = if let Some(speedometer) = &self.speedometer {
      Bot::create_from_object_with_speedometer(object, Arc::clone(speedometer))
    } else {
      Bot::create_from_object(object)
    };

    self.bots.push(Arc::new(bot));
  }

  /// Метод запуска роя, блокирующий поток на время запуска
  pub async fn launch(&mut self, server_host: impl Into<String>, server_port: u16, join_delay: u64) {
    let host = server_host.into();

    for bot in &self.bots {
      let handle = bot.connect_with_handle(&host, server_port);
      self.handles.push(handle);
      tokio::time::sleep(Duration::from_millis(join_delay)).await;
    }
  }

  /// Метод мгновенного запуска роя (без задержки)
  pub fn instant_launch(&mut self, server_host: impl Into<String>, server_port: u16) {
    let host = server_host.into();

    for bot in &self.bots {
      let handle = bot.connect_with_handle(&host, server_port);
      self.handles.push(handle);
    }
  }

  /// Метод **тихого** запуска роя (не блокирует текущий поток).
  /// Важно понимать что он **НЕ** добавляет хэндлы подключений ботов,
  /// соответственно любое взаимодействие с ними будет невозможным,
  /// так же **могут быть проблемы** при остановке роя (редко и
  /// только если выполняются долгие блокирующие операции с подключениями)
  pub fn quiet_launch(&mut self, server_host: impl Into<String>, server_port: u16, join_delay: u64) {
    let host = server_host.into();

    let mut bots = Vec::with_capacity(self.bots.len());

    for bot in &self.bots {
      bots.push(Arc::clone(bot));
    }

    tokio::spawn(async move {
      for bot in bots {
        bot.connect_with_handle(&host, server_port);
        tokio::time::sleep(Duration::from_millis(join_delay)).await;
      }
    });
  }

  /// Метод выключения и очистки роя
  pub async fn shutdown(&mut self) -> io::Result<()> {
    for handle in &self.handles {
      handle.abort();
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    // По сути все задачи ботов, связанные с подключением, должны уничтожиться
    // и соответственно все `NurtexConnection` должны быть доступны для записи
    for bot in &self.bots {
      bot.shutdown().await?;
    }

    self.handles.clear();
    self.bots.clear();

    if let Some(speedometer) = &self.speedometer {
      speedometer.stop();
    }

    Ok(())
  }

  /// Метод отмены всех хэндлов, если нужно корректно
  /// и полноценно остановить рой, используй метод `shutdown`
  pub fn abort_handles(&self) {
    for handle in &self.handles {
      handle.abort();
    }
  }
}

#[cfg(test)]
mod tests {
  use std::{io, time::Duration};

  use crate::swarm::{Swarm, SwarmObject};

  #[tokio::test]
  async fn test_default() -> io::Result<()> {
    let mut swarm = Swarm::create();
    for i in 0..=6 {
      swarm.add_object(SwarmObject::create(format!("nurtex_{}", i), "1.21.11"));
    }
    swarm.launch("localhost", 25565, 500).await;
    tokio::time::sleep(Duration::from_secs(3)).await;
    swarm.shutdown().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
  }

  #[tokio::test]
  async fn test_instant() -> io::Result<()> {
    let mut swarm = Swarm::create();
    for i in 0..=6 {
      swarm.add_object(SwarmObject::create(format!("nurtex_{}", i), "1.21.11"));
    }
    swarm.instant_launch("localhost", 25565);
    tokio::time::sleep(Duration::from_secs(3)).await;
    swarm.shutdown().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
  }

  #[tokio::test]
  async fn test_quiet() -> io::Result<()> {
    let mut swarm = Swarm::create();
    for i in 0..=6 {
      swarm.add_object(SwarmObject::create(format!("nurtex_{}", i), "1.21.11"));
    }
    swarm.quiet_launch("localhost", 25565, 500);
    tokio::time::sleep(Duration::from_secs(5)).await;
    swarm.shutdown().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(())
  }
}
