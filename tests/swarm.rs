#[cfg(test)]
mod tests {
  use std::io;
  use std::time::Duration;

  use nurtex::common::{AutoReconnectPlugin, BotCommand, BotPlugins};
  use nurtex::events::EventInvoker;
  use nurtex::swarm::SwarmObject;
  use nurtex::time::sleep;
  use nurtex::{create_shared_swarm, destroy_shared_swarm, launch_shared_swarm};

  #[tokio::test]
  async fn launch_swarm() -> io::Result<()> {
    let mut objects = Vec::new();

    for i in 0..=100 {
      let mut event_invoker = EventInvoker::new();

      event_invoker.on_spawn(async |terminal| {
        println!("Бот {} заспавнился!", terminal.receiver);
      });

      event_invoker.on_chat(async |terminal, payload| {
        println!(
          "Бот {} получил сообщение: {}",
          terminal.receiver, payload.message
        );
      });

      let object = SwarmObject::new(format!("bot_{}", i))
        .set_event_invoker(event_invoker)
        .set_plugins(BotPlugins {
          auto_reconnect: AutoReconnectPlugin {
            enabled: true,
            reconnect_delay: 1000,
          },
          ..Default::default()
        });

      objects.push(object);
    }

    let swarm = create_shared_swarm(objects);

    launch_shared_swarm(swarm.clone(), "localhost".to_string(), 25565, 25);

    sleep(8000).await;

    {
      let guard = swarm.read().await;
      guard.send(BotCommand::Chat("Test".to_string())).await;
    }

    sleep(5000).await;

    {
      let guard = swarm.read().await;
      let shared_storage = guard.shared_storage.read().await;

      for (id, entity) in &shared_storage.entities {
        println!("{} - {:?}", id, entity);
      }

      println!("Общее число сущностей: {}", shared_storage.entities.len());
    }

    sleep(60000).await;

    destroy_shared_swarm(swarm, Duration::from_millis(5000)).await?;

    Ok(())
  }
}
