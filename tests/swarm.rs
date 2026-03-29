#[cfg(test)]
mod tests {
  use std::io;
  use std::time::Duration;

  use nurtex::core::common::BotCommand;
  use nurtex::core::swarm::SwarmObject;
  use nurtex::utils::sleep;
  use nurtex::{create_shared_swarm, destroy_shared_swarm, launch_shared_swarm};

  #[tokio::test]
  async fn launch_swarm() -> io::Result<()> {
    let mut objects = Vec::new();

    for i in 0..=50 {
      let object = SwarmObject::new(format!("bot_{}", i));
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
        println!("{} - {:?}", id, entity); // DEBUG
      }

      println!("Total entity count: {}", shared_storage.entities.len()); // DEBUG
    }

    sleep(5000).await;

    destroy_shared_swarm(swarm, Duration::from_secs(5)).await?;

    Ok(())
  }
}
