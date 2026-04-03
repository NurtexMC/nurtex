#![allow(dead_code)]

use std::io;
use std::sync::Arc;

use azalea_protocol::connect::Proxy;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::common::BotInformation;
use crate::core::bot::Bot;
use crate::core::common::{BotCommand, BotPlugins, BotTerminal};
use crate::core::data::{Storage, StorageLock};
use crate::core::events::EventInvoker;
use crate::utils::time::sleep;

pub struct Swarm {
  /// Список всех ботов, после запуска данный список будет пустым
  pub bots: Vec<Bot>,

  /// Список всех терминалов, используется для управления определёнными ботами
  pub terminals: Vec<Arc<BotTerminal>>,

  /// Список всех задач (задач подключений)
  pub handles: Vec<JoinHandle<io::Result<()>>>,

  /// Shared-хранилище роя, в нём хранятся общие данные ботов о мире
  pub shared_storage: StorageLock,
}

pub type SharedSwarm = Arc<RwLock<Swarm>>;

impl Swarm {
  pub fn new() -> Self {
    Self {
      bots: Vec::new(),
      terminals: Vec::new(),
      handles: Vec::new(),
      shared_storage: Arc::new(RwLock::new(Storage::new())),
    }
  }

  /// Метод добавления объекта бота в рой
  pub fn add_object(&mut self, object: SwarmObject) {
    let mut bot = Bot::new(&object.username)
      .set_connection_timeout(object.connection_timeout)
      .set_uuid(object.uuid)
      .set_plugins(object.plugins)
      .set_information(object.information);

    if object.use_shared_storage {
      bot = bot.set_shared_storage(self.shared_storage.clone());
    }

    if let Some(proxy) = object.proxy {
      bot = bot.set_proxy(proxy);
    }

    if let Some(invoker) = object.event_invoker {
      bot = bot.set_event_invoker(invoker);
    }

    let terminal = Arc::clone(&bot.terminal);

    self.bots.push(bot);
    self.terminals.push(terminal);
  }

  /// Метод получения бота по его юзернейму
  pub fn get_bot(&self, username: &str) -> Option<&Bot> {
    self.bots.iter().find(|b| b.username == username)
  }

  /// Метод получение мутабельной ссылки на бота по его юзернейму
  pub fn get_bot_mut(&mut self, username: &str) -> Option<&mut Bot> {
    self.bots.iter_mut().find(|b| b.username == username)
  }

  /// Метод, запускающий всех ботов из роя, который блокирует поток на время запуска
  pub async fn launch_blocking(&mut self, server_host: &str, server_port: u16, join_delay: u64) {
    let bots = std::mem::take(&mut self.bots);

    for bot in bots {
      self.handles.push(bot.spawn(server_host, server_port));
      sleep(join_delay).await;
    }
  }

  /// Метод отправки команды всем ботам из роя
  pub async fn send(&self, command: BotCommand) {
    for terminal in &self.terminals {
      terminal.send(command.clone()).await;
    }
  }

  /// Метод отправки команды определённому боту из роя
  pub async fn send_to(&self, username: &str, command: BotCommand) {
    for terminal in &self.terminals {
      if terminal.receiver.as_str() == username {
        terminal.send(command).await;
        break;
      }
    }
  }

  /// Метод очистки и выключения роя
  pub async fn destroy(&mut self) {
    for terminal in &self.terminals {
      terminal.send(BotCommand::Disconnect).await;
    }

    sleep(1000).await;

    for handle in &self.handles {
      handle.abort();
    }

    self.bots.clear();
    self.terminals.clear();
    self.handles.clear();
  }

  /// Метод принудительного уничтожения роя без ожидания
  pub fn force_destroy(&mut self) {
    for handle in &self.handles {
      handle.abort();
    }

    self.bots.clear();
    self.terminals.clear();
    self.handles.clear();
  }
}

/// Объект роя, выполняющий роль **вспомогательной структуры**, которая содержит информацию.
/// Данный объект **НЕ является** полноценным ботом для роя, это лишь обёртка над его поверхностной
/// информацией (проще говоря опции).
pub struct SwarmObject {
  /// Юзернейм объекта бота
  pub username: String,

  uuid: Uuid,
  plugins: BotPlugins,
  event_invoker: Option<EventInvoker>,
  connection_timeout: u64,
  proxy: Option<Proxy>,
  information: BotInformation,
  use_shared_storage: bool,
}

impl SwarmObject {
  pub fn new(username: String) -> Self {
    Self {
      username,
      uuid: Uuid::nil(),
      plugins: BotPlugins::default(),
      event_invoker: None,
      connection_timeout: 14000,
      proxy: None,
      information: BotInformation::default(),
      use_shared_storage: true,
    }
  }

  /// Метод установки UUID
  pub fn set_uuid(mut self, uuid: Uuid) -> Self {
    self.uuid = uuid;
    self
  }

  /// Метод установки плагинов
  pub fn set_plugins(mut self, plugins: BotPlugins) -> Self {
    self.plugins = plugins;
    self
  }

  /// Метод установки инициатора событий
  pub fn set_event_invoker(mut self, invoker: EventInvoker) -> Self {
    self.event_invoker = Some(invoker);
    self
  }

  /// Метод установки таймаута подключения
  pub fn set_connection_timeout(mut self, timeout: u64) -> Self {
    self.connection_timeout = timeout;
    self
  }

  /// Метод установки информации
  pub fn set_information(mut self, information: BotInformation) -> Self {
    self.information = information;
    self
  }

  /// Метод установки прокси
  pub fn set_proxy(mut self, proxy: Proxy) -> Self {
    self.proxy = Some(proxy);
    self
  }

  /// Метод установки значения для флага `use_shared_storage`,
  /// который отвечат за использование Shared-хранилища ботами
  pub fn set_use_shared_storage(mut self, state: bool) -> Self {
    self.use_shared_storage = state;
    self
  }
}
