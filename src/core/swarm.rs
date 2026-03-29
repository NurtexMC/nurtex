#![allow(dead_code)]

use std::io;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::core::bot::{Bot, BotCommand, BotPlugins};
use crate::utils::sleep;

pub struct Swarm {
  pub bots: Vec<Bot>,
  pub handles: Vec<JoinHandle<io::Result<()>>>,
}

pub type SharedSwarm = Arc<RwLock<Swarm>>;

impl Swarm {
  pub fn new() -> Self {
    Self {
      bots: Vec::new(),
      handles: Vec::new(),
    }
  }

  /// Метод добавления бота в рой.
  pub fn add_bot(&mut self, username: &str, plugins: BotPlugins) {
    let bot = Bot::new(username).set_plugins(plugins);
    self.bots.push(bot);
  }

  /// Метод получения бота по его юзернейму.
  pub fn get_bot(&self, username: &str) -> Option<&Bot> {
    self.bots.iter().find(|b| b.username == username)
  }

  /// Метод получение мутабельной ссылки на бота по его юзернейму.
  pub fn get_bot_mut(&mut self, username: &str) -> Option<&mut Bot> {
    self.bots.iter_mut().find(|b| b.username == username)
  }

  /// Метод, запускающий всех ботов из роя, который блокирует поток на время запуска.
  pub async fn launch_blocking(&mut self, server_host: &str, server_port: u16, join_delay: u64) {
    let bots = std::mem::take(&mut self.bots);

    for bot in bots {
      self.handles.push(bot.spawn(server_host, server_port));
      sleep(join_delay).await;
    }
  }

  /// Метод отправки команды всем ботам из роя.
  pub async fn send(&self, command: BotCommand) {
    for bot in &self.bots {
      bot.terminal.send(command.clone()).await;
    }
  }

  /// Метод отправки команды определённому боту из роя.
  pub async fn send_to(&self, username: &str, command: BotCommand) {
    for bot in &self.bots {
      if bot.username == username {
        bot.terminal.send(command).await;
        break;
      }
    }
  }

  /// Метод очистки и выключения роя.
  pub async fn destroy(&mut self) {
    for bot in &self.bots {
      bot.terminal.send(BotCommand::Disconnect).await;
    }

    self.bots.clear();
    self.handles.clear();
  }
}

#[derive(Debug)]
pub struct SwarmObject {
  pub username: String,
  pub uuid: Option<Uuid>,
  pub plugins: BotPlugins,
}

impl SwarmObject {
  pub fn new(username: String) -> Self {
    Self {
      username,
      uuid: None,
      plugins: BotPlugins::default(),
    }
  }

  /// Метод установки UUID.
  pub fn set_uuid(mut self, uuid: Uuid) -> Self {
    self.uuid = Some(uuid);
    self
  }

  /// Метод установки плагинов.
  pub fn set_plugins(mut self, plugins: BotPlugins) -> Self {
    self.plugins = plugins;
    self
  }
}
