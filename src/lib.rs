use std::sync::Arc;

use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::{
  core::{
    bot::{Bot, BotPlugins},
    swarm::{SharedSwarm, Swarm, SwarmObject},
    terminal::Terminal,
  },
  utils::sleep,
};

pub mod core;
pub mod utils;

/// Вспомогательная функция создания offline-бота.
pub fn create_bot(username: &str, plugins: BotPlugins) -> (Bot, Terminal) {
  let (tx, rx) = mpsc::channel(100);

  // Нулевой UUID (00000000-0000-0000-0000-000000000000) для offline-режима.
  let bot = Bot::new(username, Uuid::nil(), rx).set_plugins(plugins);

  let terminal = Terminal {
    username: username.to_string(),
    sender: tx,
  };

  (bot, terminal)
}

/// Вспомогательная функция создания роя offline-ботов.
pub fn create_swarm(objects: Vec<SwarmObject>) -> (Swarm, Vec<Bot>) {
  let mut bots = Vec::new();
  let mut terminals = Vec::new();

  for object in objects {
    let (bot, terminal) = create_bot(&object.username, object.plugins);
    bots.push(bot);
    terminals.push(terminal);
  }

  let swarm = Swarm {
    terminals,
    handles: Vec::new(),
  };

  (swarm, bots)
}

/// Вспомогательная функция создания shared-роя offline-ботов.
pub fn create_shared_swarm(objects: Vec<SwarmObject>) -> (SharedSwarm, Vec<Bot>) {
  let (swarm, bots) = create_swarm(objects);
  (Arc::new(RwLock::new(swarm)), bots)
}

/// Вспомогательная функция неблокирующего запуска shared-роя ботов на сервер.
pub fn launch_shared_swarm(
  swarm: SharedSwarm,
  bots: Vec<Bot>,
  server_host: String,
  server_port: u16,
  join_delay: u64,
) {
  tokio::spawn(async move {
    if join_delay > 0 {
      for mut bot in bots {
        let host = server_host.clone();

        let handle = tokio::spawn(async move { bot.connect_to(&host, server_port).await });

        swarm.write().await.handles.push(handle);

        sleep(join_delay).await;
      }
    } else {
      let mut handles = Vec::new();

      for mut bot in bots {
        let host = server_host.clone();
        handles.push(tokio::spawn(async move {
          bot.connect_to(&host, server_port).await
        }));
      }

      swarm.write().await.handles.extend(handles);
    }
  });
}
