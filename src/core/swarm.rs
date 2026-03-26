#![allow(dead_code)]

use std::io;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::core::bot::{Bot, BotPlugins};
use crate::core::terminal::{Command, Terminal};
use crate::utils::sleep;

pub struct Swarm {
  pub terminals: Vec<Terminal>,
  pub handles: Vec<JoinHandle<io::Result<()>>>,
}

pub type SharedSwarm = Arc<RwLock<Swarm>>;

impl Swarm {
  /// Метод, запускающий ботов из роя и блокирующий поток на время запуска.
  pub async fn launch_blocking(
    &mut self,
    bots: Vec<Bot>,
    server_host: String,
    server_port: u16,
    join_delay: u64,
  ) {
    for mut bot in bots {
      let host = server_host.clone();

      self.handles.push(tokio::spawn(async move {
        bot.connect_to(&host, server_port).await
      }));

      sleep(join_delay).await;
    }
  }

  /// Метод отправки команды всем ботам из роя.
  pub async fn send(&self, command: Command) {
    for terminal in &self.terminals {
      terminal.send(command.clone()).await;
    }
  }

  /// Метод отправки команды определённому боту из роя.
  pub async fn send_to(&self, username: &str, command: Command) {
    for terminal in &self.terminals {
      if terminal.username.as_str() == username {
        terminal.send(command).await;
        break;
      }
    }
  }

  /// Метод очистки и выключения роя.
  pub async fn destroy(&mut self) {
    for terminal in &self.terminals {
      terminal.send(Command::Disconnect).await;
    }

    self.terminals.clear();
  }
}

#[derive(Debug)]
pub struct SwarmObject {
  pub username: String,
  pub plugins: BotPlugins,
}

impl SwarmObject {
  pub fn new(username: String) -> Self {
    Self {
      username: username,
      plugins: BotPlugins::default(),
    }
  }
}
